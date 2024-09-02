use crate::post::repository::PostRepository;
use entity::post;
use sea_orm::DatabaseConnection;

use super::db_mocks::prepare_mock_db;

#[tokio::test]
async fn test_find_post_by_id() {
    let db: &DatabaseConnection = &prepare_mock_db();

    let post = PostRepository::find_post_by_id(db, 1).await.unwrap().unwrap();
    assert_eq!(post.id, 1);

    let post = PostRepository::find_post_by_id(db, 5).await.unwrap().unwrap();
    assert_eq!(post.id, 5);
}

#[tokio::test]
async fn test_create_post() {
    let db: &DatabaseConnection = &prepare_mock_db();

    let post = PostRepository::create_post(
        db,
        post::Model {
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
            id: sea_orm::ActiveValue::Unchanged(6),
            title: sea_orm::ActiveValue::Unchanged("Title D".to_owned()),
            text: sea_orm::ActiveValue::Unchanged("Text D".to_owned())
        }
    );
}

#[tokio::test]
async fn test_update_post_by_id() {
    let db: &DatabaseConnection = &prepare_mock_db();

    let post = PostRepository::update_post_by_id(
        db,
        1,
        post::Model {
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
            id: 1,
            title: "New Title A".to_owned(),
            text: "New Text A".to_owned(),
        }
    );
}

#[tokio::test]
async fn test_delete_post() {
    let db: &DatabaseConnection = &prepare_mock_db();

    let result = PostRepository::delete_post(db, 5).await.unwrap();
    assert_eq!(result.rows_affected, 1);
}

#[tokio::test]
async fn test_delete_all_posts() {
    let db: &DatabaseConnection = &prepare_mock_db();

    let result = PostRepository::delete_all_posts(db).await.unwrap();
    assert_eq!(result.rows_affected, 5);
}
