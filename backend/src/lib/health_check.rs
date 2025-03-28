use axum::response::IntoResponse;
use axum_extra::json;

/// A simple health check endpoint for the API
#[tracing::instrument(ret)]
pub async fn health_check() -> impl IntoResponse {
    json!({"status": "ok"})
}
