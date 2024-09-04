use domain::post::service::PostService;
use domain::Post;
use entity::post;
use sea_orm::DatabaseConnection;

mod db_mocks;
use db_mocks::prepare_mock_db;

#[tokio::test]
async fn test_find_post_by_id() {
    let db: DatabaseConnection = prepare_mock_db();

    let post = PostService::find_post_by_id(&db, 1).await.unwrap().unwrap();
    assert_eq!(post.id, 1);

    let post = PostService::find_post_by_id(&db, 5).await.unwrap().unwrap();
    assert_eq!(post.id, 5);
}

#[tokio::test]
async fn test_create_post() {
    let db: DatabaseConnection = prepare_mock_db();

    let post = PostService::create_post(
        &db,
        Post {
            id: 0,
            title: "Title D".to_owned(),
            text: "Text D".to_owned(),
        },
    )
    .await
    .unwrap();

    assert_eq!(
        post,
        post::ActiveModel {
            id: sea_orm::ActiveValue::Unchanged(1),
            title: sea_orm::ActiveValue::Unchanged("Title A".to_owned()),
            text: sea_orm::ActiveValue::Unchanged("Text A".to_owned())
        }
    );
}

#[tokio::test]
async fn test_update_post_by_id() {
    let db: DatabaseConnection = prepare_mock_db();

    let post = PostService::update_post_by_id(
        &db,
        1,
        Post {
            id: 1,
            title: "New Title A".to_owned(),
            text: "New Text A".to_owned(),
        },
    )
    .await
    .unwrap();

    assert_eq!(
        post,
        post::Model {
            id: 5,
            title: "Title C".to_owned(),
            text: "Text C".to_owned(),
        }
    );
}

#[tokio::test]
async fn test_delete_post() {
    let db: DatabaseConnection = prepare_mock_db();

    let result = PostService::delete_post(&db, 5).await.unwrap();
    assert_eq!(result.rows_affected, 1);
}

#[tokio::test]
async fn test_delete_all_posts() {
    let db: DatabaseConnection = prepare_mock_db();

    let result = PostService::delete_all_posts(&db).await.unwrap();
    assert_eq!(result.rows_affected, 1);
}
