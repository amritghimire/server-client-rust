use std::path::Path;
use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use axum::routing::get_service;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;

pub fn app(static_dir: Option<String>) -> Router {
    let mut router = Router::new().route("/api/health_check", get(health_check));
    if let Some(s) = static_dir {
        router = router.fallback(
            Router::new().nest(
                "/", get_service(
                    ServeDir::new(&s)
                        .fallback(ServeFile::new(Path::new(&s).join("index.html")))
                ).handle_error(|error| async move {
                    tracing::error!(?error, "failed serving static file");
                    StatusCode::INTERNAL_SERVER_ERROR
                })
                    .layer(TraceLayer::new_for_http())
            )
        )
    }
    router
}

// Health check handler
async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "Ok")
}
