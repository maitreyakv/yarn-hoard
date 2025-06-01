use axum::http::StatusCode;
use entity::{confirmations, users};
use frontend::ApiClientError;
use sea_orm::EntityTrait;
use secrecy::SecretString;

use crate::helpers::TestApp;

#[tokio::test]
async fn create_inserts_new_user_when_ok() {
    let app = TestApp::new().await;
    app.api_client
        .create_user(
            "test@example.com",
            &SecretString::new("somePassword".into()),
        )
        .await
        .unwrap();
    let user = app.find_exactly_one(users::Entity::find()).await;
    assert_eq!(user.email, "test@example.com");
    assert_eq!(user.hashed_password.len(), 64);
    assert_eq!(user.salt.len(), 8);
    assert!(!user.is_activated);
}

#[tokio::test]
async fn create_inserts_new_confirmation_when_ok() {
    let app = TestApp::new().await;
    app.api_client
        .create_user(
            "test@example.com",
            &SecretString::new("somePassword".into()),
        )
        .await
        .unwrap();
    let user = app.find_exactly_one(users::Entity::find()).await;
    let confirmation = app.find_exactly_one(confirmations::Entity::find()).await;
    assert_eq!(confirmation.user_id, user.id);
    assert_eq!(confirmation.token.len(), 32);
}

#[tokio::test]
async fn create_returns_409_when_user_already_exists() {
    let error = TestApp::new()
        .await
        .with_unactivated_user()
        .await
        .api_client
        .create_user(
            "test@example.com",
            &SecretString::new("somePassword".into()),
        )
        .await
        .unwrap_err();
    assert!(matches!(
        error,
        ApiClientError::ErrorStatusCode(StatusCode::CONFLICT)
    ))
}

#[tokio::test]
async fn create_returns_err_when_email_is_empty() {
    let error = TestApp::new()
        .await
        .api_client
        .create_user("", &SecretString::new("somePassword".into()))
        .await
        .unwrap_err();
    assert!(matches!(
        error,
        ApiClientError::ErrorStatusCode(StatusCode::BAD_REQUEST)
    ))
}

#[tokio::test]
async fn create_returns_err_when_password_is_shorter_than_8_characters() {
    let error = TestApp::new()
        .await
        .api_client
        .create_user("test@example.com", &SecretString::new("1234567".into()))
        .await
        .unwrap_err();
    assert!(matches!(
        error,
        ApiClientError::ErrorStatusCode(StatusCode::BAD_REQUEST)
    ))
}
