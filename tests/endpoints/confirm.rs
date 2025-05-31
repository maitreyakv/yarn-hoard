//use entity::{confirmations, users};
//use sea_orm::EntityTrait;
//
//use crate::helpers::TestApp;

#[tokio::test]
async fn put_returns_200_when_ok() {
    //let app = TestApp::new().await.with_unactivated_user().await;
    //let confirmation_token = app
    //    .find_exactly_one(confirmations::Entity::find())
    //    .await
    //    .token;
    //app.put_v1(&format!("confirm/{confirmation_token}"))
    //    .await
    //    .assert_status(200);
    todo!()
}

#[tokio::test]
async fn put_returns_404_with_no_matching_token() {
    //let app = TestApp::new().await;
    //app.put_v1("confirm/missingToken").await.assert_status(404);
    todo!()
}

#[tokio::test]
async fn put_activates_user_when_ok() {
    //let app = TestApp::new().await.with_unactivated_user().await;
    //let confirmation_token = app
    //    .find_exactly_one(confirmations::Entity::find())
    //    .await
    //    .token;
    //app.put_v1(&format!("confirm/{confirmation_token}")).await;
    //let user = app.find_exactly_one(users::Entity::find()).await;
    //assert!(user.is_activated);
    todo!()
}

#[tokio::test]
async fn put_deletes_confirmation_when_ok() {
    //let app = TestApp::new().await.with_unactivated_user().await;
    //let confirmation_token = app
    //    .find_exactly_one(confirmations::Entity::find())
    //    .await
    //    .token;
    //app.put_v1(&format!("confirm/{confirmation_token}")).await;
    //app.assert_found_none(confirmations::Entity::find()).await;
    todo!()
}
