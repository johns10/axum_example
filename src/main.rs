use domain::db;
use std::env;
use web::server;

#[tokio::main]
mod config;

use crate::config::Settings;

async fn main() -> anyhow::Result<()> {
    let settings = Settings::new().expect("Failed to load settings");

    let conn = db::initialize(&settings.database_url).await?;
    let app = server::create_app(conn)?;

    server::start(app, &host, &port).await?;

    Ok(())
}
