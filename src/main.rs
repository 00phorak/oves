mod handlers;
mod logging;
mod routes;

use crate::routes::configure_router;

use tracing::{dispatcher::set_global_default, event, Level};

use tracing_log::LogTracer;

#[tokio::main]
async fn main() {
    // Load configuration
    // let cfg = confy::load_path(Path::new("config.toml"));
    // Initialize logger
    LogTracer::init().expect("Failed to init logger");

    // Set up tracing subscriber
    let subscriber = logging::configure_tracing();
    set_global_default(subscriber.into()).expect("Failed to set subscriber");

    // Start the server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    event!(Level::INFO, "Started on 127.0.0.1:8080");

    axum::serve(listener, configure_router()).await.unwrap();
}
