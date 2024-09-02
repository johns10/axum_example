mod flash;
mod post;
mod router;

use axum_example_service::sea_orm::{Database, DatabaseConnection};
use migration::{Migrator, MigratorTrait};
use std::env;
use std::sync::Arc;
use tera::Tera;

#[tokio::main]
async fn start() -> anyhow::Result<()> {
    env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");
    Migrator::up(&conn, None).await.unwrap();

    let templates = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"))
        .expect("Tera initialization failed");

    let state = AppState {
        templates,
        conn: Arc::new(conn),
    };

    let app = router::create_router(state);

    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();
    axum::serve(listener, app).await?;

    Ok(())
}

pub struct AppState {
    conn: Arc<DatabaseConnection>,
    templates: Tera,
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        Self {
            conn: Arc::clone(&self.conn),
            templates: self.templates.clone(),
        }
    }
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
