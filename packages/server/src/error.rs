use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tracing::warn;

/// Handle 404 errors with logging
pub async fn handle_404() -> Response {
    warn!("404 Not Found - Request for non-existent resource");
    (StatusCode::NOT_FOUND, "Resource not found").into_response()
}
