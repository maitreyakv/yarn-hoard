use axum::{
    Router,
    routing::{get, post},
};
use thiserror::Error;
use tower_http::trace::TraceLayer;

use crate::{
    database::{DbError, connect_to_database_and_run_migrations},
    health_check::health_check,
    users::create_user,
};

/// Build the Axum application for the API.
#[tracing::instrument]
pub async fn build_app() -> Result<Router, AppError> {
    let _pg_pool = connect_to_database_and_run_migrations().await?;

    Ok(Router::new()
        .nest(
            "/api",
            Router::new().nest(
                "/v1",
                Router::new()
                    .route("/health", get(health_check))
                    .route("/users", post(create_user)),
            ),
        )
        .layer(TraceLayer::new_for_http()))
}

/// An error that can occur while the application is running
#[derive(Debug, Error)]
pub enum AppError {
    /// The application could not interact with the database
    #[error("Error with database! > {0}")]
    DbError(#[from] DbError),
}
