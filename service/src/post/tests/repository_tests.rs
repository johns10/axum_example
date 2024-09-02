use crate::post::repository::PostRepository;
use entity::post;
use sea_orm::DatabaseConnection;

use super::db_mocks::prepare_mock_db;

#[tokio::test]
async fn test_post_operations() {
    let db: &DatabaseConnection = &prepare_mock_db();

    // Test find_post_by_id
    {
        let post = PostRepository::find_post_by_id(db, 1).await.unwrap().unwrap();
        assert_eq!(post.id, 1);
    }

    {
        let post = PostRepository::find_post_by_id(db, 5).await.unwrap().unwrap();
        assert_eq!(post.id, 5);
    }

    // Test create_post
    {
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

    // Test update_post_by_id
    {
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

    // Test delete_post
    {
        let result = PostRepository::delete_post(db, 5).await.unwrap();
        assert_eq!(result.rows_affected, 1);
    }

    // Test delete_all_posts
    {
        let result = PostRepository::delete_all_posts(db).await.unwrap();
        assert_eq!(result.rows_affected, 5);
    }
}
