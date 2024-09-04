use domain::db;
use std::env;
use web::server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");

    let conn = db::initialize(&db_url).await?;
    let app = server::create_app(conn)?;

    server::start(app, &host, &port).await?;

    Ok(())
}
