use axum::{Router, routing::get};

use crate::health_check::health_check;

/// Build the Axum application for the API.
pub fn build_app() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health", get(health_check))
}
