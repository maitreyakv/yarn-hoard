use axum::{http::StatusCode, response::IntoResponse};
use axum_extra::json;
use tracing::info;

/// A simple health check endpoint for the API
pub async fn health_check() -> impl IntoResponse {
    info!("Healthy!");
    (StatusCode::OK, json!({"status": "ok"}))
}
