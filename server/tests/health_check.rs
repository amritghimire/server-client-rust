use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use sqlx::PgPool;

mod utils;

#[sqlx::test]
async fn health_check_works(db: PgPool) {
    let response = utils::run_with_app(
        db,
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
