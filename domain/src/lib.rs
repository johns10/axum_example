pub mod post {
    pub mod model;
    pub mod service;
}

pub use post::model::Post;
pub use post::service::PostService;
pub use sea_orm;
