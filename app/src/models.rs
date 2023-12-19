use axum::http::StatusCode;
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tracing::instrument;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Worker {
    pub id: String,
    pub hostname: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub role: AppRole,
    pub port: u16,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            role: AppRole::COORDINATOR,
            port: 8080,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AppRole {
    COORDINATOR,
    WORKER,
}

#[instrument]
#[debug_handler]
pub async fn health() -> StatusCode {
    StatusCode::OK
}

#[derive(Debug, Clone, Serialize)]
pub struct AppStatus {
    pub status: String,
    pub worker_stats: WorkerStats,
}

#[derive(Debug, Clone, Serialize)]
pub struct WorkerStats {
    pub workers: Vec<Worker>,
    pub found: usize,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub workers: Arc<Mutex<Vec<Worker>>>,
    pub len: Arc<Mutex<usize>>,
}
