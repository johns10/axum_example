use crate::post::model::{Post, PostForm};
use crate::post::repository::PostRepository;
use ::entity::post;
use sea_orm::*;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum PostServiceError {
    NotFound,
    DatabaseError(String),
    // Add more error variants as needed
}

impl fmt::Display for PostServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PostServiceError::NotFound => write!(f, "Post not found"),
            PostServiceError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
        }
    }
}

impl Error for PostServiceError {}

impl From<DbErr> for PostServiceError {
    fn from(err: DbErr) -> Self {
        PostServiceError::DatabaseError(err.to_string())
    }
}

pub struct PostService<'a> {
    repository: &'a dyn PostRepository,
}

impl<'a> PostService<'a> {
    pub fn new(repository: &'a dyn PostRepository) -> Self {
        Self { repository }
    }

    pub async fn find_post_by_id(&self, id: i32) -> Result<Post, PostServiceError> {
        self.repository
            .find_post_by_id(id)
            .await?
            .map(Post::from)
            .ok_or(PostServiceError::NotFound)
    }

    pub async fn find_posts_in_page(
        &self,
        page: u64,
        posts_per_page: u64,
    ) -> Result<(Vec<Post>, u64), PostServiceError> {
        let (posts, num_pages) = self
            .repository
            .find_posts_in_page(page, posts_per_page)
            .await?;
        Ok((posts.into_iter().map(Post::from).collect(), num_pages))
    }

    pub async fn create_post(&self, form_data: PostForm) -> Result<Post, PostServiceError> {
        let post_model = post::Model {
            id: 0, // This will be ignored by the repository
            title: form_data.title,
            text: form_data.text,
        };
        let active_model = self.repository.create_post(post_model).await?;
        Ok(Post::from(active_model))
    }

    pub async fn update_post_by_id(
        &self,
        id: i32,
        form_data: PostForm,
    ) -> Result<Post, PostServiceError> {
        let post_model = post::Model {
            id,
            title: form_data.title,
            text: form_data.text,
        };
        let updated_model = self.repository.update_post_by_id(id, post_model).await?;
        Ok(Post::from(updated_model))
    }

    pub async fn delete_post(&self, id: i32) -> Result<(), PostServiceError> {
        self.repository.delete_post(id).await?;
        Ok(())
    }

    pub async fn delete_all_posts(&self) -> Result<(), PostServiceError> {
        self.repository.delete_all_posts().await?;
        Ok(())
    }
}
