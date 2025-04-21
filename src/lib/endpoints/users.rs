use axum::extract::State;
use entity::{confirmations, users};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set, TransactionTrait};
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;

use crate::app::AppError;
use crate::jsonapi::JsonApiCreate;

#[tracing::instrument(ret, err, skip(db))]
pub async fn create_user(
    State(db): State<DatabaseConnection>,
    json: JsonApiCreate<UserCreate>,
) -> Result<(), AppError> {
    db.transaction::<_, (), sea_orm::DbErr>(|txn| {
        Box::pin(async move {
            let user = users::ActiveModel::from(json.into_inner())
                .save(txn)
                .await?;

            let _confirmation = confirmations::ActiveModel {
                user_id: user.id,
                token: Set(generate_confirmation_token()),
                ..Default::default()
            }
            .save(txn)
            .await?;

            // TODO: Send email

            Ok(())
        })
    })
    .await?;

    Ok(())
}

#[derive(Debug, Deserialize)]
pub(crate) struct UserCreate {
    email: String,
    password: SecretString,
}

impl From<UserCreate> for users::ActiveModel {
    fn from(user: UserCreate) -> Self {
        let salt = crate::auth::generate_salt();
        let hashed_password = crate::auth::hash_password(&user.password, &salt);
        users::ActiveModel {
            email: Set(user.email.to_owned()),
            hashed_password: Set(hashed_password.expose_secret().to_owned()),
            salt: Set(salt.expose_secret().to_owned()),
            is_activated: Set(false),
            ..Default::default()
        }
    }
}

fn generate_confirmation_token() -> String {
    let mut token_bytes = [0; 16].to_vec();
    openssl::rand::rand_bytes(&mut token_bytes).unwrap();
    hex::encode(token_bytes)
}
