use entity::{confirmations, users};
use sea_orm::EntityTrait;

use crate::helpers::{AssertResponse, TestApp, jsonapi_create};

#[tokio::test]
async fn post_returns_200_when_ok() {
    TestApp::new()
        .await
        .post_v1_json(
            "users",
            jsonapi_create!("user", {
                "email": "test@example.com",
                "password": "somePassword"
            }),
        )
        .await
        .assert_status(200);
}

#[tokio::test]
async fn post_inserts_new_user_when_ok() {
    let app = TestApp::new().await;
    app.post_v1_json(
        "users",
        jsonapi_create!("user", {
            "email": "test@example.com",
            "password": "somePassword"
        }),
    )
    .await;
    let user = app.find_exactly_one(users::Entity::find()).await;
    assert_eq!(user.email, "test@example.com");
    assert_eq!(user.hashed_password.len(), 64);
    assert_eq!(user.salt.len(), 8);
    assert!(!user.is_activated);
}

#[tokio::test]
async fn post_inserts_new_confirmation_when_ok() {
    let app = TestApp::new().await;
    app.post_v1_json(
        "users",
        jsonapi_create!("user", {
            "email": "test@example.com",
            "password": "somePassword"
        }),
    )
    .await;
    let user = app.find_exactly_one(users::Entity::find()).await;
    let confirmation = app.find_exactly_one(confirmations::Entity::find()).await;
    assert_eq!(confirmation.user_id, user.id);
    assert_eq!(confirmation.token.len(), 32);
}

#[tokio::test]
async fn post_returns_400_when_missing_email() {
    TestApp::new()
        .await
        .post_v1_json(
            "users",
            jsonapi_create!("user", {
                "password": "somePassword"
            }),
        )
        .await
        .assert_status(400);
}

#[tokio::test]
async fn post_returns_422_when_email_is_empty() {
    TestApp::new()
        .await
        .post_v1_json(
            "users",
            jsonapi_create!("user", {
                "email": "",
                "password": "somePassword"
            }),
        )
        .await
        .assert_status(422);
}

#[tokio::test]
async fn post_returns_400_when_missing_password() {
    TestApp::new()
        .await
        .post_v1_json(
            "users",
            jsonapi_create!("user", {
                "email": "test@example.com",
            }),
        )
        .await
        .assert_status(400);
}

#[tokio::test]
async fn post_returns_422_when_password_is_shorter_than_8_characters() {
    TestApp::new()
        .await
        .post_v1_json(
            "users",
            jsonapi_create!("user", {
                "email": "test@example.com",
                "password": "1234567"
            }),
        )
        .await
        .assert_status(422);
}
