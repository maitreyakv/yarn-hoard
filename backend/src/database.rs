use std::env;

use sea_orm::{Database, DatabaseConnection};
use thiserror::Error;

use migration::{Migrator, MigratorTrait};

/// Connect to the database and run all migrations on it
#[tracing::instrument(err)]
pub async fn connect_to_db_and_run_migrations() -> Result<DatabaseConnection, DbError> {
    let db = Database::connect(env::var("DATABASE_URL")?).await?;
    Migrator::up(&db, None).await?;
    Ok(db)
}

// An error that occurs while interacting with the database
#[derive(Debug, Error)]
pub enum DbError {
    #[error("Cannot read DATABASE_URL environment variable!")]
    DbUrlEnvVar(#[from] std::env::VarError),

    /// An issue with connecting to the database
    #[error("{0}")]
    DbErr(#[from] sea_orm::DbErr),
}
