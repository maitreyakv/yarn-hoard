use sea_orm_migration::{prelude::*, schema::*};

use crate::m20250318_233214_create_users_table::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Confirmations::Table)
                    .col(pk_auto(Confirmations::Id))
                    .col(integer_uniq(Confirmations::UserId))
                    .col(string_uniq(Confirmations::Token))
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("confirmations_user_id_fkey")
                    .from(Confirmations::Table, Confirmations::UserId)
                    .to(Users::Table, Users::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("confirmations_token_idx")
                    .table(Confirmations::Table)
                    .col(Confirmations::Token)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Confirmations::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Confirmations {
    Table,
    Id,
    UserId,
    Token,
}
