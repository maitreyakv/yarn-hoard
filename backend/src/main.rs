use yarn_hoard_api::build_app;

#[tokio::main]
async fn main() {
    let app = build_app();

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
