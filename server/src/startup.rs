use crate::routes::{health_check, subscriptions};

use axum::routing::get_service;
use axum::{http::StatusCode, routing::get, Extension, Router};

use crate::configuration::Settings;
use crate::State;
use sqlx::PgPool;
use std::path::Path;
use std::sync::Arc;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;

pub async fn app(settings: Settings, db: Option<PgPool>) -> Router {
    let connection = if let Some(database) = db {
        database
    } else {
        PgPool::connect(&settings.database.connection_string())
            .await
            .expect("Failed to connect to Postgres")
    };

    sqlx::migrate!()
        .run(&connection)
        .await
        .expect("Failed to perform migration");

    let mut router = Router::new()
        .route("/api/health_check", get(health_check))
        .merge(subscriptions::router());
    if !settings.application.static_dir.is_empty() {
        router = router.fallback(
            Router::new().nest(
                "/",
                get_service(ServeDir::new(&settings.application.static_dir).fallback(
                    ServeFile::new(Path::new(&settings.application.static_dir).join("index.html")),
                ))
                .handle_error(|error| async move {
                    tracing::error!(?error, "failed serving static file");
                    StatusCode::INTERNAL_SERVER_ERROR
                })
                .layer(TraceLayer::new_for_http()),
            ),
        )
    }

    let shared_settings = Arc::new(settings);
    let shared_database = Arc::new(State {
        database: connection,
    });

    router
        .layer(Extension(shared_database))
        .layer(Extension(shared_settings))
}
