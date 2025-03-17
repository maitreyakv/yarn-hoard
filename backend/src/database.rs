use sqlx::{PgPool, postgres::PgConnectOptions};
use thiserror::Error;
use tracing::{debug, error, info};

/// Connect to the database and run all migrations on it
#[tracing::instrument]
pub async fn connect_to_database_and_run_migrations() -> Result<PgPool, DbError> {
    debug!("Connecting to the database...");
    let pg_pool = PgPool::connect_with(PgConnectOptions::new())
        .await
        .inspect_err(|error| error!(?error))?;
    info!("Connected to the database");

    debug!("Migrating the database...");
    sqlx::migrate!("./migrations")
        .run(&pg_pool)
        .await
        .inspect_err(|error| error!(?error))?;
    info!("Migrated the database");

    Ok(pg_pool)
}

/// An error that occurs while interacting with the database
#[derive(Debug, Error)]
pub enum DbError {
    /// An issue with connecting to the database
    #[error("Failed to connect to the database! > {0}")]
    ConnectionFailure(#[from] sqlx::Error),

    /// An issue with migrating the database
    #[error("Failed to migrate database! > {0}")]
    MigrationFailure(#[from] sqlx::migrate::MigrateError),
}
