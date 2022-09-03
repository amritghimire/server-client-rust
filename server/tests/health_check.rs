use axum::{
    body::Body,
    http::{Request, StatusCode},
};

use tower::ServiceExt;

#[tokio::test]
async fn health_check_works() {
    let app = server::app(None);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/health_check")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    assert_eq!(&body[..], b"Ok");
}
