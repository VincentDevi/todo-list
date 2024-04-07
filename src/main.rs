mod templates;
use axum::{routing::get, Router};
use templates::*;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(hello))
        .nest_service("/assets", tower_http::services::ServeDir::new("assets"));

    tracing::info!("Server running...");

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
