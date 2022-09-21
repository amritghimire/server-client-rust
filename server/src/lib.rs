mod error;

use axum::extract::rejection::JsonRejection;
use axum::routing::get_service;
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::path::Path;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;

pub type JsonPayload<T> = Result<Json<T>, JsonRejection>;
pub type JsonResponse<T> = Result<Json<T>, error::AppError>;

pub fn app(static_dir: Option<String>) -> Router {
    let mut router = Router::new()
        .route("/api/health_check", get(health_check))
        .route("/api/subscriptions", post(subscribe));
    if let Some(s) = static_dir {
        router = router.fallback(
            Router::new().nest(
                "/",
                get_service(
                    ServeDir::new(&s).fallback(ServeFile::new(Path::new(&s).join("index.html"))),
                )
                .handle_error(|error| async move {
                    tracing::error!(?error, "failed serving static file");
                    StatusCode::INTERNAL_SERVER_ERROR
                })
                .layer(TraceLayer::new_for_http()),
            ),
        )
    }
    router
}

// Health check handler
async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "Ok")
}

#[derive(Serialize, Deserialize)]
struct SubscribeUser {
    email: String,
    name: String,
}

async fn subscribe(payload: JsonPayload<SubscribeUser>) -> JsonResponse<SubscribeUser> {
    let Json(user) = payload?;
    Ok(user.into())
}
