use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use std::net::SocketAddr;

pub async fn run() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app().into_make_service())
        .await
        .unwrap();
}

pub fn app() -> Router {
    Router::new().route("/health_check", get(health_check))
}

// Health check handler
async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "Ok")
}
