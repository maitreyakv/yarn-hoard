use axum::{http::StatusCode, response::IntoResponse};
use axum_extra::json;

pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, json!({"status": "ok"}))
}
