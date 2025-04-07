use api::AppConfig;
use sea_orm::{Database, DatabaseConnection, EntityTrait, PrimaryKeyTrait, Select};
use testcontainers_modules::{
    postgres::Postgres,
    testcontainers::{ContainerAsync, runners::AsyncRunner},
};

pub struct TestApp {
    _close_tx: tokio::sync::oneshot::Sender<()>,
    app_port: u16,
    api_client: reqwest::Client,
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

        let app = api::build_app(AppConfig {
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
            app_port,
            api_client: reqwest::Client::new(),
            _db_container: container,
            db: Database::connect(db_url).await.unwrap(),
        }
    }

    pub async fn with_unactivated_user(self) -> Self {
        self.post_v1_json(
            "users",
            jsonapi_create!("user", {
                "email": "test@example.com",
                "password": "somePassword"
            }),
        )
        .await;
        self
    }

    pub async fn post_v1_json(&self, path: &str, json: serde_json::Value) -> reqwest::Response {
        self.api_client
            .post(format!(
                "http://localhost:{}/api/v1/{}",
                self.app_port, path
            ))
            .json(&json)
            .send()
            .await
            .unwrap()
    }

    pub async fn put_v1(&self, path: &str) -> reqwest::Response {
        self.api_client
            .put(format!(
                "http://localhost:{}/api/v1/{}",
                self.app_port, path
            ))
            .send()
            .await
            .unwrap()
    }

    pub async fn find_exactly_one<E: EntityTrait>(&self, select: Select<E>) -> E::Model
    where
        <<E as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: From<i32>,
    {
        let rows = select.all(&self.db).await.unwrap();
        assert!(rows.len().eq(&1));
        rows.into_iter().next().unwrap()
    }

    pub async fn assert_found_none<E: EntityTrait>(&self, select: Select<E>) {
        let rows = select.all(&self.db).await.unwrap();
        assert!(rows.len().eq(&0));
    }
}

macro_rules! jsonapi_create {
    ($t:literal, $a:tt) => {
        ::serde_json::json!({
            "data": {
                "type": $t,
                "attributes": $a
            }
        })
    };
}

pub(crate) use jsonapi_create;

pub trait AssertResponse {
    fn assert_status(self, status: u16) -> Self;
}

impl AssertResponse for reqwest::Response {
    fn assert_status(self, status: u16) -> Self {
        assert_eq!(
            self.status(),
            reqwest::StatusCode::from_u16(status).unwrap()
        );
        self
    }
}
