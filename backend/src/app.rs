use axum::{
    Router,
    routing::{get, post},
};

use crate::{health_check::health_check, users::create_user};

/// Build the Axum application for the API.
pub fn build_app() -> Router {
    Router::new().nest(
        "/api",
        Router::new().nest(
            "/v1",
            Router::new()
                .route("/health", get(health_check))
                .route("/users", post(create_user)),
        ),
    )
}
