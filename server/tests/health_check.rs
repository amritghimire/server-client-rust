use axum::{
    body::Body,
    http::{Request, StatusCode},
};

mod utils;

#[tokio::test]
async fn health_check_works() {
    let response = utils::run_with_app(
        Request::builder()
            .uri("/api/health_check")
            .body(Body::empty())
            .unwrap(),
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    assert_eq!(&body[..], b"Ok");
}
