use axum::response::{IntoResponse, Json};

/// A simple health check endpoint for the API
pub async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({ "status": "ok" }))
}
