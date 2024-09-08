use crate::post::model::Post;
use ::entity::posts;
use async_trait::async_trait;
use chrono::Utc;
use sea_orm::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PostRepositoryError {
    #[error("Post not found")]
    NotFound,
    #[error("Database error: {0}")]
    DatabaseError(String),
    // Add more error variants as needed
}

impl From<DbErr> for PostRepositoryError {
    fn from(err: DbErr) -> Self {
        PostRepositoryError::DatabaseError(err.to_string())
    }
}

#[async_trait]
pub trait PostRepository: Send + Sync {
    async fn find_post_by_id(&self, id: i32) -> Result<Option<Post>, PostRepositoryError>;
    async fn find_posts_in_page(
        &self,
        page: u64,
        posts_per_page: u64,
    ) -> Result<(Vec<Post>, u64), PostRepositoryError>;
    async fn create_post(&self, post: Post) -> Result<Post, PostRepositoryError>;
    async fn update_post_by_id(&self, id: i32, post: Post) -> Result<Post, PostRepositoryError>;
    async fn delete_post(&self, id: i32) -> Result<u64, PostRepositoryError>;
    async fn delete_all_posts(&self) -> Result<u64, PostRepositoryError>;
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
    async fn find_post_by_id(&self, id: i32) -> Result<Option<Post>, PostRepositoryError> {
        posts::Entity::find_by_id(id)
            .one(self.conn.as_ref())
            .await
            .map(|opt_model| opt_model.map(Post::from))
            .map_err(PostRepositoryError::from)
    }

    async fn find_posts_in_page(
        &self,
        page: u64,
        posts_per_page: u64,
    ) -> Result<(Vec<Post>, u64), PostRepositoryError> {
        let paginator = posts::Entity::find()
            .order_by_asc(posts::Column::Id)
            .paginate(self.conn.as_ref(), posts_per_page);
        let num_pages = paginator.num_pages().await?;

        paginator
            .fetch_page(page - 1)
            .await
            .map(|p| (p.into_iter().map(Post::from).collect(), num_pages))
            .map_err(PostRepositoryError::from)
    }

    async fn create_post(&self, post: Post) -> Result<Post, PostRepositoryError> {
        let model = posts::ActiveModel {
            title: Set(post.title.to_owned()),
            text: Set(post.text.to_owned()),
            created_at: Set(Utc::now().naive_utc()),
            updated_at: Set(post.updated_at),
            ..Default::default()
        };
        let result = model.insert(self.conn.as_ref()).await?;
        Ok(Post::from(result))
    }

    async fn update_post_by_id(&self, id: i32, post: Post) -> Result<Post, PostRepositoryError> {
        let model = posts::ActiveModel {
            id: Set(id),
            title: Set(post.title.to_owned()),
            text: Set(post.text.to_owned()),
            created_at: Set(post.created_at),
            updated_at: Set(Utc::now().naive_utc()),
        };
        let result = model.update(self.conn.as_ref()).await?;
        Ok(Post::from(result))
    }

    async fn delete_post(&self, id: i32) -> Result<u64, PostRepositoryError> {
        posts::Entity::delete_by_id(id)
            .exec(self.conn.as_ref())
            .await
            .map(|res| res.rows_affected)
            .map_err(PostRepositoryError::from)
    }

    async fn delete_all_posts(&self) -> Result<u64, PostRepositoryError> {
        posts::Entity::delete_many()
            .exec(self.conn.as_ref())
            .await
            .map(|res| res.rows_affected)
            .map_err(PostRepositoryError::from)
    }
}
