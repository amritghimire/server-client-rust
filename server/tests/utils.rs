use axum::http::Request;
use axum::response::Response;
use axum::Router;
use hyper::Body;
use tower::{Service, ServiceExt};

use server::{configuration::Settings, startup};

use sqlx::PgPool;

pub async fn run_with_app(db: PgPool, request: Request<Body>) -> Response {
    let settings = Settings::test().expect("Unable to initialize configuration");

    let app = startup::app(settings, Some(db)).await;
    let response = app.oneshot(request).await.unwrap();
    response
}

#[allow(dead_code)]
pub async fn run_request(app: &mut Router, request: Request<Body>) -> Response {
    let response = app.ready().await.unwrap().call(request).await.unwrap();
    response
}
