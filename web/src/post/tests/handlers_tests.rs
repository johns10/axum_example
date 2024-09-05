use axum::body::to_bytes;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use domain::post::model::Post;
use domain::post::repository::PostRepository;
use domain::repository::Repository;
use mockall::predicate::*;
use std::sync::Arc;
use tower::util::ServiceExt;

use crate::post::handlers;
use crate::AppState;
use domain::post::tests::db_mocks::MockPostRepository;

fn create_app_state(mock_repo: MockPostRepository) -> AppState {
    let repository = Repository {
        post: Arc::new(mock_repo) as Arc<dyn PostRepository + Send + Sync>,
    };

    AppState {
        repository: Arc::new(repository),
        templates: tera::Tera::new("templates/**/*").unwrap(),
    }
}

#[tokio::test]
async fn test_list_posts() {
    let mut mock_repo = MockPostRepository::new();
    mock_repo
        .expect_find_posts_in_page()
        .with(eq(1), eq(5))
        .times(1)
        .returning(|_, _| {
            Ok((
                vec![
                    Post {
                        id: 1,
                        title: "Test Post 1".to_string(),
                        text: "Content 1".to_string(),
                    },
                    Post {
                        id: 2,
                        title: "Test Post 2".to_string(),
                        text: "Content 2".to_string(),
                    },
                ],
                1,
            ))
        });

    let app_state = create_app_state(mock_repo);

    let app = axum::Router::new()
        .route("/", axum::routing::get(handlers::list_posts))
        .with_state(app_state);

    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body()).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("Test Post 1"));
    assert!(body_str.contains("Test Post 2"));
}

#[tokio::test]
async fn test_create_post() {
    let mut mock_repo = MockPostRepository::new();
    mock_repo
        .expect_create_post()
        .with(function(|post: &Post| {
            post.title == "New Post" && post.text == "New Content"
        }))
        .times(1)
        .returning(|post| {
            Ok(Post {
                id: 1,
                title: post.title,
                text: post.text,
            })
        });

    let app_state = create_app_state(mock_repo);

    let app = axum::Router::new()
        .route("/", axum::routing::post(handlers::create_post))
        .with_state(app_state);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/")
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(Body::from("title=New+Post&text=New+Content"))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    assert_eq!(response.headers().get("Location").unwrap(), "/");
}

#[tokio::test]
async fn test_delete_post() {
    let mut mock_repo = MockPostRepository::new();
    mock_repo
        .expect_delete_post()
        .with(eq(1))
        .times(1)
        .returning(|_| Ok(1));

    let app_state = create_app_state(mock_repo);

    let app = axum::Router::new()
        .route("/:id", axum::routing::post(handlers::delete_post))
        .with_state(app_state);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/1")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    assert_eq!(response.headers().get("Location").unwrap(), "/");
}
