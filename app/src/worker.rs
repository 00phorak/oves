use std::sync::Arc;
use std::time::Duration;

use axum_macros::debug_handler;
use reqwest::Client;
use tokio::time::sleep;
use tracing::event;
use tracing::instrument;
use tracing::Level;

use crate::models::ProcessReport;

#[instrument]
#[debug_handler]
pub async fn process_shortest_route_chunk() {
    let reporter = Arc::new(reqwest::Client::builder().build().unwrap());
    match report_progress(&reporter).await {
        reqwest::StatusCode::OK => event!(
            Level::INFO,
            "reporting succcessful, continue with processing"
        ),
        _ => event!(
            Level::ERROR,
            "reporting progress error, attempting to fix connection"
        ),
    }

    match report_result(&reporter, 5).await {
        reqwest::StatusCode::OK => {
            event!(Level::INFO, "returned result successfully");
        }
        _ => event!(Level::ERROR, "failed to return result"),
    }
    todo!("processing something");
}

#[instrument]
pub async fn report_progress(reporter: &Arc<Client>) -> reqwest::StatusCode {
    let body = ProcessReport {
        percentage: 8,
        job_id: "todo job id".to_string(),
        worker_id: "todo worker id".to_string(),
    };
    let response = reporter
        .post("http://localhost:8080/report_progress")
        .json(&body)
        .send()
        .await
        .unwrap();
    return response.status();
}

#[instrument]
pub async fn report_result(reporter: &Arc<Client>, retry: u8) -> reqwest::StatusCode {
    let mut response = reporter
        .post("http://localhost:8080/report_result")
        .send()
        .await
        .unwrap();
    let mut count = 0;
    while response.status() != reqwest::StatusCode::OK && count <= retry {
        response = reporter
            .post("http://localhost:8080/report_result")
            .send()
            .await
            .unwrap();
        count += 1;
        sleep(Duration::from_secs(1)).await;
    }
    return response.status();
}
