use crate::post::repository::{PostRepository, PostRepositoryImpl};
use sea_orm::DatabaseConnection;

pub struct Repository<'a> {
    pub post: Box<dyn PostRepository + 'a>,
}

impl<'a> Repository<'a> {
    pub fn new(conn: &'a DatabaseConnection) -> Self {
        Self {
            post: Box::new(PostRepositoryImpl::new(conn)),
        }
    }
}
