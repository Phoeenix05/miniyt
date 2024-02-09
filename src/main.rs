use std::net::SocketAddr;

use axum::{http::Method, routing::get, Router};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

mod config;

#[tokio::main]
async fn main() {
    let config = config::load_config();

    let trace = TraceLayer::new_for_http();
    let cors = CorsLayer::new().allow_methods([Method::GET, Method::POST]);

    let app = Router::new()
        .route("/", get(|| async { "hallo" }))
        .route("/api", get(|| async { "hallo api" }))
        .layer(ServiceBuilder::new().layer(trace).layer(cors));

    let host = [0, 0, 0, 0];
    let port = config.backend.port;
    let tcp_listener = TcpListener::bind(SocketAddr::from((host, port)))
        .await
        .unwrap();
    axum::serve(tcp_listener, app).await.unwrap();
}
