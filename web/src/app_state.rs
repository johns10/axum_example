use domain::DatabaseConnection;
use std::sync::Arc;
use tera::Tera;

pub struct AppState {
    pub conn: Arc<DatabaseConnection>,
    pub templates: Tera,
}

impl AppState {
    pub fn new(conn: DatabaseConnection, templates: Tera) -> Self {
        Self {
            conn: Arc::new(conn),
            templates,
        }
    }
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        Self {
            conn: Arc::clone(&self.conn),
            templates: self.templates.clone(),
        }
    }
}
