use axum::http::StatusCode;
use axum_macros::debug_handler;
use tracing::instrument;

#[instrument]
#[debug_handler]
pub async fn health() -> StatusCode {
    StatusCode::OK
}
