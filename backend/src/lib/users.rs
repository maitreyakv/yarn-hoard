use axum::{Json, http::StatusCode};
use secrecy::SecretString;
use serde::Deserialize;

//#[tracing::instrument(ret, err)]
pub async fn create_user(Json(req): Json<CreateUser>) -> Result<(), StatusCode> {
    let user: User = req.into();
    dbg!(user);

    Ok(())
}

#[derive(Debug, Deserialize)]
struct User {
    email: String,
    hashed_password: SecretString,
    salt: SecretString,
}

crate::jsonapi::create!(User);
