use axum::{
    Router,
    extract::FromRef,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
};
use sea_orm::{DatabaseConnection, DbErr, TransactionError};
use secrecy::SecretString;

use crate::{
    database::connect_to_db_and_run_migrations,
    endpoints::{
        health_check::health_check,
        users::{confirm_user_creation, create_user},
    },
};

/// Build the Axum application for the API.
#[tracing::instrument(err)]
pub async fn build_app(config: AppConfig) -> anyhow::Result<Router> {
    let db = connect_to_db_and_run_migrations(config.database_url).await?;

    Ok(Router::new()
        .nest(
            "/api",
            Router::new().nest(
                "/v1",
                Router::new()
                    .route("/health", get(health_check))
                    .route("/users", post(create_user))
                    .route("/users/confirm/{token}", put(confirm_user_creation)),
            ),
        )
        .with_state(AppState { db })
        .layer(tower_http::cors::CorsLayer::permissive())
        .layer(tower_http::trace::TraceLayer::new_for_http()))
}

#[derive(Debug)]
pub struct AppConfig {
    pub database_url: SecretString,
}

#[derive(Clone)]
struct AppState {
    db: DatabaseConnection,
}

impl FromRef<AppState> for DatabaseConnection {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.db.clone()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    DbErr(#[from] DbErr),

    #[error(transparent)]
    TransactionError(#[from] TransactionError<DbErr>),

    #[error("{0}")]
    StatusCode(StatusCode),
}

impl From<StatusCode> for AppError {
    fn from(status: StatusCode) -> Self {
        Self::StatusCode(status)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::DbErr(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            Self::TransactionError(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
            Self::StatusCode(status) => status.into_response(),
        }
    }
}
