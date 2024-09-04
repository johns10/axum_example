use crate::{router, AppState};
use axum::Router;
use domain::DatabaseConnection;
use tera::Tera;

pub fn create_app(conn: DatabaseConnection) -> anyhow::Result<Router> {
    let templates = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"))
        .expect("Tera initialization failed");
    let state = AppState::new(conn, templates);
    Ok(router::create_router(state))
}

pub async fn start(app: Router, host: &str, port: u16) -> anyhow::Result<()> {
    let server_url = format!("{host}:{port}");
    let listener = tokio::net::TcpListener::bind(&server_url).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
