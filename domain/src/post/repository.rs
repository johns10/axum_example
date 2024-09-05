use ::entity::post;
use async_trait::async_trait;
use sea_orm::*;

#[async_trait]
pub trait PostRepository: Send + Sync {
    async fn find_post_by_id(&self, id: i32) -> Result<Option<post::Model>, DbErr>;
    async fn find_posts_in_page(
        &self,
        page: u64,
        posts_per_page: u64,
    ) -> Result<(Vec<post::Model>, u64), DbErr>;
    async fn create_post(&self, form_data: post::Model) -> Result<post::ActiveModel, DbErr>;
    async fn update_post_by_id(
        &self,
        id: i32,
        form_data: post::Model,
    ) -> Result<post::Model, DbErr>;
    async fn delete_post(&self, id: i32) -> Result<DeleteResult, DbErr>;
    async fn delete_all_posts(&self) -> Result<DeleteResult, DbErr>;
}

use std::sync::Arc;

pub struct PostRepositoryImpl {
    conn: Arc<DatabaseConnection>,
}

impl PostRepositoryImpl {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl PostRepository for PostRepositoryImpl {
    async fn find_post_by_id(&self, id: i32) -> Result<Option<post::Model>, DbErr> {
        post::Entity::find_by_id(id).one(&**self.conn).await
    }

    async fn find_posts_in_page(
        &self,
        page: u64,
        posts_per_page: u64,
    ) -> Result<(Vec<post::Model>, u64), DbErr> {
        let paginator = post::Entity::find()
            .order_by_asc(post::Column::Id)
            .paginate(&**self.conn, posts_per_page);
        let num_pages = paginator.num_pages().await?;

        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }

    async fn create_post(&self, form_data: post::Model) -> Result<post::ActiveModel, DbErr> {
        post::ActiveModel {
            title: Set(form_data.title.to_owned()),
            text: Set(form_data.text.to_owned()),
            ..Default::default()
        }
        .save(&**self.conn)
        .await
    }

    async fn update_post_by_id(
        &self,
        id: i32,
        form_data: post::Model,
    ) -> Result<post::Model, DbErr> {
        let post: post::ActiveModel = post::Entity::find_by_id(id)
            .one(&**self.conn)
            .await?
            .ok_or(DbErr::Custom("Cannot find post.".to_owned()))
            .map(Into::into)?;

        post::ActiveModel {
            id: post.id,
            title: Set(form_data.title.to_owned()),
            text: Set(form_data.text.to_owned()),
        }
        .update(&**self.conn)
        .await
    }

    async fn delete_post(&self, id: i32) -> Result<DeleteResult, DbErr> {
        let post: post::ActiveModel = post::Entity::find_by_id(id)
            .one(&**self.conn)
            .await?
            .ok_or(DbErr::Custom("Cannot find post.".to_owned()))
            .map(Into::into)?;

        post.delete(&**self.conn).await
    }

    async fn delete_all_posts(&self) -> Result<DeleteResult, DbErr> {
        post::Entity::delete_many().exec(&**self.conn).await
    }
}
