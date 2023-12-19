use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum_macros::debug_handler;
use std::sync::Arc;
use tracing::event;
use tracing::{instrument, Level};

use crate::models::{AppState, AppStatus, Worker, WorkerStats};

#[instrument(skip(state))]
#[debug_handler]
pub async fn register_worker(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Worker>,
) -> StatusCode {
    event!(Level::INFO, "Registering worker..");
    let mut data = state.workers.lock().expect("mutex was poisened");
    data.push(payload);
    let mut len = state.len.lock().expect("mutex was poisened for len");
    *len = data.len();
    event!(Level::INFO, "Worker registered..");
    StatusCode::OK
}

#[instrument(skip(state))]
#[debug_handler]
pub async fn base_path(State(state): State<Arc<AppState>>) -> Json<AppStatus> {
    event!(Level::INFO, "App status..");
    let workers = state.workers.lock().expect("mutex was poisened");
    let len = state.len.lock().expect("mutex was poisened");

    Json(AppStatus {
        status: "working".to_string(),
        worker_stats: WorkerStats {
            workers: workers.clone(),
            found: len.clone(),
        },
    })
}

#[instrument]
#[debug_handler]
pub async fn health() -> StatusCode {
    StatusCode::OK
}
