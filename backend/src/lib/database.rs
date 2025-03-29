use std::env;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use migration::{Migrator, MigratorTrait};

/// Connect to the database and run all migrations on it
#[tracing::instrument(err)]
pub async fn connect_to_db_and_run_migrations() -> anyhow::Result<DatabaseConnection> {
    let mut opts = ConnectOptions::new(env::var("DATABASE_URL")?);
    opts.sqlx_logging(false);
    let db = Database::connect(opts).await?;
    Migrator::up(&db, None).await?;
    Ok(db)
}
