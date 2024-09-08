use crate::post::model::{Post, PostForm};
use crate::post::repository::{PostRepository, PostRepositoryError};
use chrono;

#[derive(Debug)]
pub enum PostServiceError {
    NotFound,
    DatabaseError(String),
    // Add more error variants as needed
}

impl From<PostRepositoryError> for PostServiceError {
    fn from(err: PostRepositoryError) -> Self {
        match err {
            PostRepositoryError::NotFound => PostServiceError::NotFound,
            PostRepositoryError::DatabaseError(msg) => PostServiceError::DatabaseError(msg),
        }
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
            .await
            .map_err(PostServiceError::from)?
            .ok_or(PostServiceError::NotFound)
    }

    pub async fn find_posts_in_page(
        &self,
        page: u64,
        posts_per_page: u64,
    ) -> Result<(Vec<Post>, u64), PostServiceError> {
        self.repository
            .find_posts_in_page(page, posts_per_page)
            .await
            .map_err(PostServiceError::from)
    }

    pub async fn create_post(&self, form_data: PostForm) -> Result<Post, PostServiceError> {
        let now = chrono::Utc::now().naive_utc();
        let post = Post {
            id: 0, // This will be ignored by the repository
            title: form_data.title,
            text: form_data.text,
            created_at: now,
            updated_at: now,
        };
        self.repository.create_post(post).await.map_err(PostServiceError::from)
    }

    pub async fn update_post_by_id(
        &self,
        id: i32,
        form_data: PostForm,
    ) -> Result<Post, PostServiceError> {
        let now = chrono::Utc::now().naive_utc();
        let post = Post {
            id,
            title: form_data.title,
            text: form_data.text,
            created_at: now, // This should ideally be fetched from the existing post
            updated_at: now,
        };
        self.repository.update_post_by_id(id, post).await.map_err(PostServiceError::from)
    }

    pub async fn delete_post(&self, id: i32) -> Result<u64, PostServiceError> {
        self.repository.delete_post(id).await.map_err(PostServiceError::from)
    }

    pub async fn delete_all_posts(&self) -> Result<u64, PostServiceError> {
        self.repository.delete_all_posts().await.map_err(PostServiceError::from)
    }
}
