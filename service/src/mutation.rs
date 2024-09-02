use ::entity::post;
use sea_orm::*;
use crate::post::repository::PostRepository;

pub struct Mutation;

impl Mutation {
    pub async fn create_post(
        db: &DbConn,
        form_data: post::Model,
    ) -> Result<post::ActiveModel, DbErr> {
        PostRepository::create_post(db, form_data).await
    }

    pub async fn update_post_by_id(
        db: &DbConn,
        id: i32,
        form_data: post::Model,
    ) -> Result<post::Model, DbErr> {
        PostRepository::update_post_by_id(db, id, form_data).await
    }

    pub async fn delete_post(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        PostRepository::delete_post(db, id).await
    }

    pub async fn delete_all_posts(db: &DbConn) -> Result<DeleteResult, DbErr> {
        PostRepository::delete_all_posts(db).await
    }
}
