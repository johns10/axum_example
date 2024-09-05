use crate::post::repository::PostRepository;

pub struct Repository {
    pub post: Box<dyn PostRepository>,
}

impl Repository {
    pub fn new(post_repository: Box<dyn PostRepository>) -> Self {
        Self {
            post: post_repository,
        }
    }
}
