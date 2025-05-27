use sea_orm::{Database, DatabaseConnection, EntityTrait, PrimaryKeyTrait, Select};
//use secrecy::SecretString;
use testcontainers_modules::{
    postgres::Postgres,
    testcontainers::{ContainerAsync, runners::AsyncRunner},
};

use api_client::ApiClient;

pub struct TestApp {
    _close_tx: tokio::sync::oneshot::Sender<()>,
    pub api_client: api_client::ApiClient,
    _db_container: ContainerAsync<Postgres>,
    db: DatabaseConnection,
}

impl TestApp {
    pub async fn new() -> Self {
        let container = Postgres::default().start().await.unwrap();
        let db_url = format!(
            "postgres://postgres:postgres@{}:{}/postgres",
            container.get_host().await.unwrap(),
            container.get_host_port_ipv4(5432).await.unwrap(),
        );

        let app = api_server::build_app(api_server::AppConfig {
            database_url: db_url.to_owned().into(),
        })
        .await
        .unwrap();

        let (close_tx, close_rx) = tokio::sync::oneshot::channel();

        let listener = tokio::net::TcpListener::bind("0.0.0.0:0").await.unwrap();
        let app_port = listener.local_addr().unwrap().port();
        tokio::spawn(async {
            axum::serve(listener, app)
                .with_graceful_shutdown(async move {
                    let _ = close_rx.await;
                })
                .await
                .unwrap()
        });

        Self {
            _close_tx: close_tx,
            api_client: ApiClient::insecure(&format!("localhost:{app_port}")),
            _db_container: container,
            db: Database::connect(db_url).await.unwrap(),
        }
    }

    //pub async fn with_unactivated_user(self) -> Self {
    //    self.api_client
    //        .create_user(
    //            "test@example.com",
    //            &SecretString::new("somePassword".into()),
    //        )
    //        .await
    //        .unwrap();
    //    self
    //}

    pub async fn find_exactly_one<E: EntityTrait>(&self, select: Select<E>) -> E::Model
    where
        <<E as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: From<i32>,
    {
        let rows = select.all(&self.db).await.unwrap();
        assert!(rows.len().eq(&1));
        rows.into_iter().next().unwrap()
    }

    //pub async fn assert_found_none<E: EntityTrait>(&self, select: Select<E>) {
    //    let rows = select.all(&self.db).await.unwrap();
    //    assert!(rows.len().eq(&0));
    //}
}
