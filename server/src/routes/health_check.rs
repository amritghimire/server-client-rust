use axum::http::StatusCode;
use axum::response::IntoResponse;

// Health check handler
#[tracing::instrument(name = "Adding a new subscriber")]
pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "Ok")
}
