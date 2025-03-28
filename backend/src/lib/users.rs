use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use secrecy::SecretString;
use serde::Deserialize;

use crate::json_api::JsonApiCreate;

#[tracing::instrument(ret)]
pub async fn create_user(Json(payload): Json<JsonApiCreate<CreateUser>>) -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    email: String,
    hashed_password: SecretString,
    salt: SecretString,
}
