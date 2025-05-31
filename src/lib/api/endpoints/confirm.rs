use axum::extract::{Path, State};
use entity::{confirmations, users};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
    TransactionTrait,
};
use tracing::error;

use crate::api::app::AppError;

pub async fn confirm(
    State(db): State<DatabaseConnection>,
    Path(token): Path<String>,
) -> Result<(), AppError> {
    let (confirmation, user) = confirmations::Entity::find()
        .filter(confirmations::Column::Token.eq(token))
        .find_also_related(users::Entity)
        .one(&db)
        .await?
        .ok_or(AppError::NotFound)?;
    let user = user.ok_or_else(|| {
        error!(?confirmation, "Confirmation has no user!");
        AppError::InternalServerError
    })?;

    db.transaction::<_, (), sea_orm::DbErr>(|txn| {
        Box::pin(async move {
            let mut user: users::ActiveModel = user.into();
            user.is_activated = Set(true);
            user.save(txn).await?;

            let confirmation: confirmations::ActiveModel = confirmation.into();
            confirmation.delete(txn).await?;

            Ok(())
        })
    })
    .await?;

    Ok(())
}
