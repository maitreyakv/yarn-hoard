/// The main file for the `yarn-hoard-api` binary, the server for the API
///
use sqlx::PgPool;
use std::process::ExitCode;
use tracing::{debug, error, info};
use yarn_hoard_api::build_app;

#[tokio::main]
async fn main() -> ExitCode {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    debug!("Connecting to the database...");
    let pg_pool = match connect_to_db().await {
        Ok(pool) => {
            info!("Connected to the database");
            pool
        }
        Err(error) => {
            error!(?error, "Failed to connect to the database!");
            return ExitCode::FAILURE;
        }
    };

    debug!("Migrating the database...");
    match sqlx::migrate!("./migrations").run(&pg_pool).await {
        Ok(()) => {
            info!("Migrated the database");
        }
        Err(error) => {
            error!(?error, "Failed to migrate the database!");
            return ExitCode::FAILURE;
        }
    };

    let app = build_app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();

    ExitCode::SUCCESS
}

async fn connect_to_db() -> sqlx::Result<PgPool> {
    PgPool::connect("postgresql://yarn_hoard:password@db:5432/yarn_hoard").await
}
