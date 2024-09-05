use ::entity::post;
use async_trait::async_trait;
use domain::post::model::PostForm;
use domain::post::service::PostService;
use domain::PostRepository;
use mockall::mock;
use sea_orm::*;
use std::sync::Arc;

mock! {
    pub PostRepository {}

    #[async_trait]
    impl PostRepository for PostRepository {
        async fn find_post_by_id(&self, id: i32) -> Result<Option<post::Model>, DbErr>;
        async fn find_posts_in_page(&self, page: u64, posts_per_page: u64) -> Result<(Vec<post::Model>, u64), DbErr>;
        async fn create_post(&self, form_data: post::Model) -> Result<post::ActiveModel, DbErr>;
        async fn update_post_by_id(&self, id: i32, form_data: post::Model) -> Result<post::Model, DbErr>;
        async fn delete_post(&self, id: i32) -> Result<DeleteResult, DbErr>;
        async fn delete_all_posts(&self) -> Result<DeleteResult, DbErr>;
    }
}

#[tokio::test]
async fn test_find_post_by_id() {
    let mut mock_repo = MockPostRepository::new();
    mock_repo
        .expect_find_post_by_id()
        .with(mockall::predicate::eq(1))
        .times(1)
        .returning(|_| {
            Ok(Some(post::Model {
                id: 1,
                title: "Test Post".to_string(),
                text: "This is a test post".to_string(),
            }))
        });

    let service = PostService::new(&mock_repo);

    let post = service.find_post_by_id(1).await.unwrap().unwrap();
    assert_eq!(post.id, 1);
    assert_eq!(post.title, "Test Post");
    assert_eq!(post.text, "This is a test post");
}

#[tokio::test]
async fn test_create_post() {
    let mut mock_repo = MockPostRepository::new();
    mock_repo
        .expect_create_post()
        .with(mockall::predicate::function(|post: &post::Model| {
            post.title == "New Post" && post.text == "This is a new post"
        }))
        .times(1)
        .returning(|_| {
            Ok(post::ActiveModel {
                id: Set(1),
                title: Set("New Post".to_string()),
                text: Set("This is a new post".to_string()),
            })
        });

    let service = PostService::new(&mock_repo);

    let post = service
        .create_post(PostForm {
            title: "New Post".to_string(),
            text: "This is a new post".to_string(),
        })
        .await
        .unwrap();

    assert_eq!(post.id.unwrap(), 1);
    assert_eq!(post.title.unwrap(), "New Post");
    assert_eq!(post.text.unwrap(), "This is a new post");
}

#[tokio::test]
async fn test_update_post_by_id() {
    let mut mock_repo = MockPostRepository::new();
    mock_repo
        .expect_update_post_by_id()
        .with(
            mockall::predicate::eq(1),
            mockall::predicate::function(|post: &post::Model| {
                post.title == "Updated Post" && post.text == "This post has been updated"
            }),
        )
        .times(1)
        .returning(|_, _| {
            Ok(post::Model {
                id: 1,
                title: "Updated Post".to_string(),
                text: "This post has been updated".to_string(),
            })
        });

    let service = PostService::new(&mock_repo);

    let post = service
        .update_post_by_id(
            1,
            PostForm {
                title: "Updated Post".to_string(),
                text: "This post has been updated".to_string(),
            },
        )
        .await
        .unwrap();

    assert_eq!(post.id, 1);
    assert_eq!(post.title, "Updated Post");
    assert_eq!(post.text, "This post has been updated");
}

#[tokio::test]
async fn test_delete_post() {
    let mut mock_repo = MockPostRepository::new();
    mock_repo
        .expect_delete_post()
        .with(mockall::predicate::eq(1))
        .times(1)
        .returning(|_| Ok(DeleteResult { rows_affected: 1 }));

    let service = PostService::new(&mock_repo);

    let result = service.delete_post(1).await.unwrap();
    assert_eq!(result.rows_affected, 1);
}

#[tokio::test]
async fn test_delete_all_posts() {
    let mut mock_repo = MockPostRepository::new();
    mock_repo
        .expect_delete_all_posts()
        .times(1)
        .returning(|| Ok(DeleteResult { rows_affected: 5 }));

    let service = PostService::new(&mock_repo);

    let result = service.delete_all_posts().await.unwrap();
    assert_eq!(result.rows_affected, 5);
}
