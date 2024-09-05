use crate::post::model::Post;
use crate::post::repository::PostRepository;
use async_trait::async_trait;
use mockall::mock;
use mockall::predicate::*;
use sea_orm::*;

mock! {
    pub PostRepository {}

    #[async_trait]
    impl PostRepository for PostRepository {
        async fn find_post_by_id(&self, id: i32) -> Result<Option<Post>, DbErr>;
        async fn find_posts_in_page(&self, page: u64, posts_per_page: u64) -> Result<(Vec<Post>, u64), DbErr>;
        async fn create_post(&self, post: Post) -> Result<Post, DbErr>;
        async fn update_post_by_id(&self, id: i32, post: Post) -> Result<Post, DbErr>;
        async fn delete_post(&self, id: i32) -> Result<u64, DbErr>;
        async fn delete_all_posts(&self) -> Result<u64, DbErr>;
    }
}
