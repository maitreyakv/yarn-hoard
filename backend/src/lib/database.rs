use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use secrecy::{ExposeSecret, SecretString};

/// Connect to the database and run all migrations on it
#[tracing::instrument(err)]
pub async fn connect_to_db_and_run_migrations(
    database_url: SecretString,
) -> anyhow::Result<DatabaseConnection> {
    let mut opts = ConnectOptions::new(database_url.expose_secret());
    opts.sqlx_logging(false);

    let db = Database::connect(opts).await?;

    Migrator::up(&db, None).await?;

    Ok(db)
}
