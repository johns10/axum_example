use crate::PostRepository;
use ::entity::post;
use async_trait::async_trait;
use mockall::mock;
use mockall::predicate::*;
use sea_orm::*;

mock! {
    pub PostRepository {}

    #[async_trait]
    impl PostRepository for PostRepository {
        async fn find_post_by_id(&self, id: i32) -> Result<Option<post::Model>, DbErr>;
        async fn find_posts_in_page(&self, page: u64, posts_per_page: u64) -> Result<(Vec<post::Model>, u64), DbErr>;
        async fn create_post(&self, form_data: post::Model) -> Result<post::ActiveModel, DbErr>;
        async fn update_post_by_id(&self, id: i32, form_data: post::Model) -> Result<post::Model, DbErr>;
        async fn delete_post(&self, id: i32) -> Result<DeleteResult, DbErr>;
        async fn delete_all_posts(&self) -> Result<DeleteResult, DbErr>;
    }
}
