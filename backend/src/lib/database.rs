use std::env;

use sea_orm::{Database, DatabaseConnection};

use migration::{Migrator, MigratorTrait};

/// Connect to the database and run all migrations on it
#[tracing::instrument(err)]
pub async fn connect_to_db_and_run_migrations() -> anyhow::Result<DatabaseConnection> {
    let db = Database::connect(env::var("DATABASE_URL")?).await?;
    Migrator::up(&db, None).await?;
    Ok(db)
}
