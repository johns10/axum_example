use crate::post::model::Post;
use crate::post::repository::{PostRepository, PostRepositoryError};
use async_trait::async_trait;
use mockall::mock;
use mockall::predicate::*;

mock! {
    pub PostRepository {}

    #[async_trait]
    impl PostRepository for PostRepository {
        async fn find_post_by_id(&self, id: i32) -> Result<Option<Post>, PostRepositoryError>;
        async fn find_posts_in_page(&self, page: u64, posts_per_page: u64) -> Result<(Vec<Post>, u64), PostRepositoryError>;
        async fn create_post(&self, post: Post) -> Result<Post, PostRepositoryError>;
        async fn update_post_by_id(&self, id: i32, post: Post) -> Result<Post, PostRepositoryError>;
        async fn delete_post(&self, id: i32) -> Result<u64, PostRepositoryError>;
        async fn delete_all_posts(&self) -> Result<u64, PostRepositoryError>;
    }
}
