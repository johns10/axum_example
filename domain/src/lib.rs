pub mod post {
    pub mod model;
    pub mod repository;
    pub mod service;
}
pub mod db;
pub mod repository;

pub use post::model::Post;
pub use post::repository::PostRepository;
pub use post::service::PostService;

pub use sea_orm;
pub use sea_orm::DatabaseConnection;
