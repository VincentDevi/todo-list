mod handlers;
mod models;
use axum::{
    routing::{get, post},
    Router,
};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(handlers::hello))
        .route("/app", get(handlers::index))
        .route("/count/:count", post(handlers::increment_counter))
        .nest_service("/assets", tower_http::services::ServeDir::new("assets"));

    tracing::info!("Server running...");

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
