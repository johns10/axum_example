use chrono::Utc;
use domain::post::model::PostForm;
use domain::post::service::PostService;
use domain::post::tests::db_mocks::MockPostRepository;
use domain::Post;

#[tokio::test]
async fn test_find_post_by_id() {
    let mut mock_repo = MockPostRepository::new();
    mock_repo
        .expect_find_post_by_id()
        .with(mockall::predicate::eq(1))
        .times(1)
        .returning(|_| {
            Ok(Some(Post {
                id: 1,
                title: "Test Post".to_string(),
                text: "This is a test post".to_string(),
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            }))
        });

    let service = PostService::new(&mock_repo);

    let post = service.find_post_by_id(1).await.unwrap();
    assert_eq!(post.id, 1);
    assert_eq!(post.title, "Test Post");
    assert_eq!(post.text, "This is a test post");
}

#[tokio::test]
async fn test_create_post() {
    let mut mock_repo = MockPostRepository::new();
    mock_repo
        .expect_create_post()
        .with(mockall::predicate::function(|post: &Post| {
            post.title == "New Post" && post.text == "This is a new post"
        }))
        .times(1)
        .returning(|post| {
            Ok(Post {
                id: 1,
                title: post.title,
                text: post.text,
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
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

    assert_eq!(post.id, 1);
    assert_eq!(post.title, "New Post");
    assert_eq!(post.text, "This is a new post");
}

#[tokio::test]
async fn test_update_post_by_id() {
    let mut mock_repo = MockPostRepository::new();
    mock_repo
        .expect_update_post_by_id()
        .with(
            mockall::predicate::eq(1),
            mockall::predicate::function(|post: &Post| {
                post.title == "Updated Post" && post.text == "This post has been updated"
            }),
        )
        .times(1)
        .returning(|_, post| {
            Ok(Post {
                id: 1,
                title: post.title,
                text: post.text,
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
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
        .returning(|_| Ok(1));

    let service = PostService::new(&mock_repo);

    let result = service.delete_post(1).await;
    assert_eq!(result.unwrap(), 1);
}

#[tokio::test]
async fn test_delete_all_posts() {
    let mut mock_repo = MockPostRepository::new();
    mock_repo
        .expect_delete_all_posts()
        .times(1)
        .returning(|| Ok(5));

    let service = PostService::new(&mock_repo);

    let result = service.delete_all_posts().await;
    assert_eq!(result.unwrap(), 5);
}
