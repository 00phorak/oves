use axum::{routing::get, Router};
use axum_prometheus::PrometheusMetricLayer;

use super::handlers::*;

pub fn configure_router() -> Router {
    // Set up Prometheus metrics
    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();
    Router::new()
        .route("/health", get(health))
        .route(
            "/metrics",
            get(move || async move { metric_handle.render() }),
        )
        .layer(prometheus_layer)
}
