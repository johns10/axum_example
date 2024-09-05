use domain::{db, repository::Repository};
use web::server;

mod config;

use crate::config::Settings;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let settings = Settings::new().expect("Failed to load settings");

    let conn = db::initialize(&settings.database_url).await?;
    let repository = Repository::new(&conn);
    let app = server::create_app(repository)?;

    server::start(app, &settings.host, settings.port).await?;

    Ok(())
}
