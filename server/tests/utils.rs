use axum::http::Request;
use axum::response::Response;
use axum::Router;
use hyper::Body;
use tower::{Service, ServiceExt};

pub async fn run_with_app(request: Request<Body>) -> Response {
    let app = server::startup::app(None);
    let response = app.oneshot(request).await.unwrap();
    response
}

pub async fn run_request(app: &mut Router, request: Request<Body>) -> Response {
    let response = app.ready().await.unwrap().call(request).await.unwrap();
    response
}
