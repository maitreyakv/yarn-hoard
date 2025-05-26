mod assert;
mod run;
mod setup;

use sea_orm::DatabaseConnection;
use testcontainers_modules::{postgres::Postgres, testcontainers::ContainerAsync};

pub struct TestApp {
    _close_tx: tokio::sync::oneshot::Sender<()>,
    pub app_port: u16,
    api_client: reqwest::Client,
    _db_container: ContainerAsync<Postgres>,
    db: DatabaseConnection,
}
