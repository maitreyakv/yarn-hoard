use api_server::AppConfig;
use sea_orm::Database;
use testcontainers_modules::{postgres::Postgres, testcontainers::runners::AsyncRunner};

use crate::helpers::TestApp;
use crate::helpers::jsonapi_create;

impl TestApp {
    pub async fn new() -> Self {
        let container = Postgres::default().start().await.unwrap();
        let db_url = format!(
            "postgres://postgres:postgres@{}:{}/postgres",
            container.get_host().await.unwrap(),
            container.get_host_port_ipv4(5432).await.unwrap(),
        );

        let app = api_server::build_app(AppConfig {
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
}
