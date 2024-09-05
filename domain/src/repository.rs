use std::sync::Arc;
use crate::post::repository::{PostRepository, PostRepositoryImpl};
use sea_orm::DatabaseConnection;

pub struct Repository {
    pub post: Arc<dyn PostRepository>,
}

impl Repository {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self {
            post: Arc::new(PostRepositoryImpl::new(conn)),
        }
    }
}
