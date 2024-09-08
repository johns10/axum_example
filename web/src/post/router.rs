use axum::{
    routing::{get, post},
    Router,
};

use crate::post::handlers;
use crate::AppState;

pub fn create_post_router() -> Router<AppState> {
    Router::new()
        .route("/", get(handlers::list_posts).post(handlers::create_post))
        .route("/:id", post(handlers::update_post))
        .route("/:id/edit", get(handlers::edit_post))
        .route("/new", get(handlers::new_post))
        .route("/:id/delete", post(handlers::delete_post))
}
