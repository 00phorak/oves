mod config;
mod handlers;
mod logging;
mod models;
mod routes;

use std::sync::{Arc, Mutex};
use tracing::{dispatcher::set_global_default, event, Level};

use models::AppState;
use tracing_log::LogTracer;

use crate::routes::configure_routes;

#[tokio::main]
async fn main() {
    // Load configuration
    let cfg = config::load_config().unwrap();

    // Initialize logger
    LogTracer::init().expect("Failed to init logger");

    // Set up tracing subscriber
    let subscriber = logging::configure_tracing();
    set_global_default(subscriber.into()).expect("Failed to set subscriber");

    // Set up shared state
    let shared_state = Arc::new(AppState {
        workers: Arc::new(Mutex::new(Vec::new().to_owned())),
        len: Arc::new(Mutex::new(0)),
    });

    // Start the server
    let hostname = format!("127.0.0.1:{}", &cfg.port);
    let listener = tokio::net::TcpListener::bind(&hostname).await.unwrap();
    event!(
        Level::INFO,
        "Started {:?} node on {:?}",
        &cfg.role,
        &hostname
    );
    axum::serve(listener, configure_routes(cfg.role, shared_state))
        .await
        .unwrap();
}
