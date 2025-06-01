use entity::{confirmations, users};
use sea_orm::EntityTrait;

use axum::http::StatusCode;
use frontend::ApiClientError;

use crate::helpers::TestApp;

#[tokio::test]
async fn put_returns_404_with_no_matching_token() {
    let app = TestApp::new().await;
    let error = app
        .api_client
        .confirm_user_creation("missingToken")
        .await
        .unwrap_err();
    assert!(matches!(
        error,
        ApiClientError::ErrorStatusCode(StatusCode::NOT_FOUND)
    ));
}

#[tokio::test]
async fn put_activates_user_when_ok() {
    let app = TestApp::new().await.with_unactivated_user().await;
    let token = app
        .find_exactly_one(confirmations::Entity::find())
        .await
        .token;
    app.api_client.confirm_user_creation(&token).await.unwrap();
    let user = app.find_exactly_one(users::Entity::find()).await;
    assert!(user.is_activated);
}

#[tokio::test]
async fn put_deletes_confirmation_when_ok() {
    let app = TestApp::new().await.with_unactivated_user().await;
    let token = app
        .find_exactly_one(confirmations::Entity::find())
        .await
        .token;
    app.api_client.confirm_user_creation(&token).await.unwrap();
    app.assert_found_none(confirmations::Entity::find()).await;
}
