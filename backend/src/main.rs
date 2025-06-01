/// The main file for the `yarn-hoard-api` binary, the server for the API
use backend::{AppConfig, build_app};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let config = AppConfig {
        database_url: std::env::var("DATABASE_URL")
            .expect("DATABASE_URL is not set!")
            .into(),
    };

    let app = build_app(config).await.unwrap();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
