use crate::routes::{health_check, subscribe};

use axum::routing::get_service;
use axum::{
    http::StatusCode,
    routing::{get, post},
    Router,
};

use std::path::Path;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;

pub fn app(static_dir: Option<String>) -> Router {
    // sqlx::migrate!().run(<&your_pool OR &mut your_connection>).await?;
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
