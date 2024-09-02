
use axum::{
    routing::{get, get_service, post},
    Router,
};
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

use crate::{post::handlers, AppState};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(handlers::list_posts).post(handlers::create_post))
        .route("/:id", get(handlers::edit_post).post(handlers::update_post))
        .route("/new", get(handlers::new_post))
        .route("/delete/:id", post(handlers::delete_post))
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
