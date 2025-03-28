use axum::{
    Router,
    routing::{get, post},
};
use tower_http::trace::TraceLayer;

use crate::{
    database::connect_to_db_and_run_migrations, health_check::health_check, users::create_user,
};

/// Build the Axum application for the API.
#[tracing::instrument(err)]
pub async fn build_app() -> anyhow::Result<Router> {
    let _db = connect_to_db_and_run_migrations().await?;

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
