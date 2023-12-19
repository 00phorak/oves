use axum::{
    routing::{get, post},
    Router,
};
use axum_prometheus::PrometheusMetricLayer;
use std::sync::Arc;

use crate::models::{AppRole, AppState};

use super::handlers::*;

pub fn configure_routes(role: AppRole, shared_state: Arc<AppState>) -> Router {
    // Set up Prometheus metrics
    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();
    match role {
        AppRole::COORDINATOR => Router::new()
            .route("/", get(base_path))
            .route("/health", get(health))
            .route("/register_worker", post(register_worker))
            .route(
                "/metrics",
                get(move || async move { metric_handle.render() }),
            )
            .layer(prometheus_layer)
            .with_state(shared_state),
        AppRole::WORKER => Router::new()
            .route("/", get(base_path))
            .route("/health", get(health))
            .route(
                "/metrics",
                get(move || async move { metric_handle.render() }),
            )
            .layer(prometheus_layer)
            .with_state(shared_state),
    }
}
