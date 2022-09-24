use axum::http::StatusCode;
use axum::{
    body::Body,
    http::{header, Request},
};
use serde_json::json;

mod utils;

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let body = json!({
        "name": "Amrit Ghimire",
        "email": "amrit@example.com"
    });

    let mut database = utils::get_database().await;
    let response = utils::run_with_app(
        Request::post("/api/subscriptions")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(serde_json::to_vec(&body).unwrap()))
            .unwrap(),
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&mut database)
        .await
        .expect("Failed to fetch saved subscription");

    assert_eq!(saved.email, "amrit@example.com");

    assert_eq!(saved.name, "Amrit Ghimire");
}

#[tokio::test]
async fn subscribe_returns_a_400_for_missing_form_data() {
    let mut app = server::startup::app(None);

    let test_cases = vec![
        (json!({"name": "Amrit Ghimire"}), "missing the email"),
        (json!({"email": "amrit@example.com"}), "missing the name"),
        (json!({}), "missing both name and email"),
    ];

    for (body, error_message) in test_cases {
        let response = utils::run_request(
            &mut app,
            Request::post("/api/subscriptions")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_vec(&body).unwrap()))
                .unwrap(),
        )
        .await;
        assert_eq!(
            response.status(),
            StatusCode::BAD_REQUEST,
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}
