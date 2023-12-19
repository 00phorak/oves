use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

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
pub struct ProcessReport {
    pub job_id: String,
    pub percentage: u8,
    pub worker_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AppRole {
    COORDINATOR,
    WORKER,
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
