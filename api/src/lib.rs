pub mod flash;
pub mod post;
pub mod router;

use axum_example_service::sea_orm::DatabaseConnection;
use std::sync::Arc;
use tera::Tera;

pub struct AppState {
    pub conn: Arc<DatabaseConnection>,
    pub templates: Tera,
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        Self {
            conn: Arc::clone(&self.conn),
            templates: self.templates.clone(),
        }
    }
}
