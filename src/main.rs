use domain::db;
use web::server;

mod config;

use crate::config::Settings;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let settings = Settings::new().expect("Failed to load settings");

    let conn = db::initialize(&settings.database_url).await?;
    let app = server::create_app(conn)?;

    server::start(app, &settings.host, settings.port).await?;

    Ok(())
}
