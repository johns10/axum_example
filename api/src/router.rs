use axum::{routing::get_service, Router};
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

use crate::{post::router::create_post_router, AppState};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .nest("/posts", create_post_router())
        .nest_service(
            "/static",
            get_service(ServeDir::new(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/static"
            )))
            .handle_error(|error| async move {
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {error}"),
                )
            }),
        )
        .layer(CookieManagerLayer::new())
        .with_state(state)
}
