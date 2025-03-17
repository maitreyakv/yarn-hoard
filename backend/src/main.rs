/// The main file for the `yarn-hoard-api` binary, the server for the API
use tracing::error;

use yarn_hoard_api::build_app;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let app = build_app()
        .await
        .inspect_err(|error| error!(%error))
        .unwrap();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .inspect_err(|error| error!(%error))
        .unwrap();

    axum::serve(listener, app)
        .await
        .inspect_err(|error| error!(%error))
        .unwrap();
}
