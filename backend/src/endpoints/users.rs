use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use entity::{confirmations, users};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
    TransactionTrait,
};
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use tracing::error;

use crate::app::AppError;
use crate::jsonapi::JsonApiCreate;

pub async fn create_user(
    State(db): State<DatabaseConnection>,
    user_create: JsonApiCreate<UserCreate>,
) -> Result<StatusCode, AppError> {
    // Deserialize and validate the request body
    let user = users::ActiveModel::try_from(user_create.into_inner())?;

    // Return 409 Conflict if email already has an account
    if users::Entity::find()
        .filter(users::Column::Email.eq(user.email.to_owned().unwrap()))
        .one(&db)
        .await?
        .is_some()
    {
        return Err(StatusCode::CONFLICT.into());
    };

    // Open a transaction...
    db.transaction::<_, (), sea_orm::DbErr>(|txn| {
        Box::pin(async move {
            // ...first create the new user record...
            let user = user.save(txn).await?;

            // ...then create a confirmation token record...
            let _confirmation = confirmations::ActiveModel {
                user_id: user.id,
                token: Set(generate_confirmation_token()),
                ..Default::default()
            }
            .save(txn)
            .await?;

            // TODO: Send email
            // ...and finally send the confirmation email with the token.

            Ok(())
        })
    })
    .await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn confirm_user_creation(
    State(db): State<DatabaseConnection>,
    Path(token): Path<String>,
) -> Result<StatusCode, AppError> {
    // Find the User and Confirmation records
    let (confirmation, user) = confirmations::Entity::find()
        .filter(confirmations::Column::Token.eq(token))
        .find_also_related(users::Entity)
        .one(&db)
        .await?
        .ok_or(StatusCode::NOT_FOUND)?;
    let user = user.ok_or_else(|| {
        error!(?confirmation, "Confirmation has no user!");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Start a transaction and...
    db.transaction::<_, (), sea_orm::DbErr>(|txn| {
        Box::pin(async move {
            // ...first set the user as active...
            let mut user: users::ActiveModel = user.into();
            user.is_activated = Set(true);
            user.save(txn).await?;

            // ...finally delete the confirmation token record.
            let confirmation: confirmations::ActiveModel = confirmation.into();
            confirmation.delete(txn).await?;

            Ok(())
        })
    })
    .await?;

    Ok(StatusCode::NO_CONTENT)
}

#[derive(Debug, Deserialize)]
pub(crate) struct UserCreate {
    email: String,
    password: SecretString,
}

impl TryFrom<UserCreate> for users::ActiveModel {
    type Error = AppError;

    fn try_from(user: UserCreate) -> Result<Self, Self::Error> {
        let salt = crate::auth::generate_salt();
        let hashed_password = crate::auth::hash_password(&user.password, &salt);

        if user.email.is_empty() {
            return Err(StatusCode::BAD_REQUEST.into());
        }

        if user.password.expose_secret().len().lt(&8) {
            return Err(StatusCode::BAD_REQUEST.into());
        }

        Ok(users::ActiveModel {
            email: Set(user.email.to_owned()),
            hashed_password: Set(hashed_password.expose_secret().to_owned()),
            salt: Set(salt.expose_secret().to_owned()),
            is_activated: Set(false),
            ..Default::default()
        })
    }
}

fn generate_confirmation_token() -> String {
    let mut token_bytes = [0; 16].to_vec();
    openssl::rand::rand_bytes(&mut token_bytes).unwrap();
    hex::encode(token_bytes)
}
