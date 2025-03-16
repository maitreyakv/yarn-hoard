/// The main file for the `yarn-hoard-api` binary, the server for the API
use yarn_hoard_api::build_app;

#[tokio::main]
async fn main() {
    let app = build_app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
