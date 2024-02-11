mod config;
mod data;
mod error;
mod integration;
mod routes;
mod utility;

use std::net::SocketAddr;

use axum::{http::Method, response::Redirect, routing::get, Router};
use notify::RecursiveMode;
use notify_debouncer_mini::new_debouncer;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};
use tower_livereload::LiveReloadLayer;

use crate::routes::api;

#[tokio::main]
async fn main() {
    env_logger::builder().init();
    let config = config::load_config();

    // –––––––––– layers ––––––––––
    let trace = TraceLayer::new_for_http();
    let cors = CorsLayer::new().allow_methods([Method::GET, Method::POST]);
    let live_reload = LiveReloadLayer::new();
    // reload helper
    let reloader = live_reload.reloader();

    // –––––––––– router ––––––––––
    let app = Router::new()
        // routes
        .nest("/api", Router::new())
        .nest(
            "/api/channel",
            Router::new().route("/:cuid", get(api::channel::by_cuid)),
        )
        // static content
        .nest_service(
            "/",
            ServeDir::new("./web/dist")
                // fixes page reload issues
                .fallback(get(|| async { Redirect::to("/") })),
        )
        // layers
        .layer(
            ServiceBuilder::new()
                .layer(live_reload)
                .layer(trace)
                .layer(cors),
        );

    // –––––––––– live reload ––––––––––
    let mut debouncer = new_debouncer(
        std::time::Duration::from_millis(250),
        move |res| match res {
            Ok(_) => reloader.reload(),
            Err(e) => eprintln!("{:?}", e),
        },
    )
    .unwrap();

    debouncer
        .watcher()
        .watch(std::path::Path::new("./web/dist"), RecursiveMode::Recursive)
        .unwrap();

    // –––––––––– serve ––––––––––
    let (host, port) = ([127, 0, 0, 1], config.backend.port);
    log::debug!("{:?}, {}", host, port);

    let tcp_listener = TcpListener::bind(SocketAddr::from((host, port)))
        .await
        .unwrap();
    axum::serve(tcp_listener, app).await.unwrap();
}
