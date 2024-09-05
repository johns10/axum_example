use crate::post::model::{Post, PostForm};
use crate::post::repository::PostRepository;
use ::entity::post;
use sea_orm::*;

pub struct PostService<'a> {
    repository: &'a dyn PostRepository,
}

impl<'a> PostService<'a> {
    pub fn new(repository: &'a dyn PostRepository) -> Self {
        Self { repository }
    }

    pub async fn find_post_by_id(&self, id: i32) -> Result<Option<Post>, DbErr> {
        self.repository
            .find_post_by_id(id)
            .await
            .map(|opt_model| opt_model.map(Post::from))
    }

    pub async fn find_posts_in_page(
        &self,
        page: u64,
        posts_per_page: u64,
    ) -> Result<(Vec<Post>, u64), DbErr> {
        let (posts, num_pages) = self
            .repository
            .find_posts_in_page(page, posts_per_page)
            .await?;
        Ok((posts.into_iter().map(Post::from).collect(), num_pages))
    }

    pub async fn create_post(&self, form_data: PostForm) -> Result<post::ActiveModel, DbErr> {
        let post_model = post::Model {
            id: 0, // This will be ignored by the repository
            title: form_data.title,
            text: form_data.text,
        };
        self.repository.create_post(post_model).await
    }

    pub async fn update_post_by_id(
        &self,
        id: i32,
        form_data: PostForm,
    ) -> Result<post::Model, DbErr> {
        let post_model = post::Model {
            id,
            title: form_data.title,
            text: form_data.text,
        };
        self.repository.update_post_by_id(id, post_model).await
    }

    pub async fn delete_post(&self, id: i32) -> Result<DeleteResult, DbErr> {
        self.repository.delete_post(id).await
    }

    pub async fn delete_all_posts(&self) -> Result<DeleteResult, DbErr> {
        self.repository.delete_all_posts().await
    }
}
