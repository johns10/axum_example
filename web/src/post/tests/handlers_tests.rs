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
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::post::handlers;
use crate::AppState;
use domain::post::tests::db_mocks::MockPostRepository;

#[ctor::ctor]
fn init() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
}

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
        .oneshot(
            Request::builder()
                .uri("/?page=1&posts_per_page=5")
                .header("Cookie", "")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("Test Post 1"));
    assert!(body_str.contains("Test Post 2"));
}

#[tokio::test]
async fn test_create_post() {
    let mut mock_repo = MockPostRepository::new();
    mock_repo
        .expect_create_post()
        .withf(|post: &Post| post.title == "New Post" && post.text == "New Content")
        .times(1)
        .returning(|post| {
            Ok(Post {
                id: 1,
                title: post.title.clone(),
                text: post.text.clone(),
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
                .uri("/posts")
                .header("Content-Type", "application/x-www-form-urlencoded")
                .header("Cookie", "")
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
        .route("/delete/:id", axum::routing::post(handlers::delete_post))
        .with_state(app_state);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/delete/1")
                .header("Cookie", "")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    assert_eq!(response.headers().get("Location").unwrap(), "/");
}
