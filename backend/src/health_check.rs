use axum::{http::StatusCode, response::IntoResponse};
use axum_extra::json;

/// A simple health check endpoint for the API
pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, json!({"status": "ok"}))
}
