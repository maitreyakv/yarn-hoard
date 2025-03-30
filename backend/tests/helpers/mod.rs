use api::AppConfig;
use testcontainers_modules::{
    postgres::Postgres,
    testcontainers::{ContainerAsync, runners::AsyncRunner},
};

pub struct TestApp {
    _close_tx: tokio::sync::oneshot::Sender<()>,
    app_port: u16,
    api_client: reqwest::Client,
    _db_container: ContainerAsync<Postgres>,
}

impl TestApp {
    pub async fn new() -> Self {
        let container = Postgres::default().start().await.unwrap();

        let app = api::build_app(AppConfig {
            database_url: format!(
                "postgres://postgres:postgres@{}:{}/postgres",
                container.get_host().await.unwrap(),
                container.get_host_port_ipv4(5432).await.unwrap(),
            )
            .into(),
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
        }
    }

    pub async fn post_v1(&self, path: &str, json: serde_json::Value) -> reqwest::Response {
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
