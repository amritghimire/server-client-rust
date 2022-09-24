use axum::http::StatusCode;
use axum::{
    body::Body,
    http::{header, Request},
};
use serde_json::json;
use server::configuration::Settings;
use sqlx::PgPool;

mod utils;

#[sqlx::test]
async fn subscribe_returns_a_200_for_valid_form_data(db: PgPool) {
    let body = json!({
        "name": "Amrit Ghimire",
        "email": "amrit@example.com"
    });

    let response = utils::run_with_app(
        db.clone(),
        Request::post("/api/subscriptions")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(serde_json::to_vec(&body).unwrap()))
            .unwrap(),
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&db)
        .await
        .expect("Failed to fetch saved subscription");

    assert_eq!(saved.email, "amrit@example.com");

    assert_eq!(saved.name, "Amrit Ghimire");

    let response = utils::run_with_app(
        db.clone(),
        Request::post("/api/subscriptions")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(serde_json::to_vec(&body).unwrap()))
            .unwrap(),
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK);
    let saved = sqlx::query!("select count(id) from subscriptions;")
        .fetch_one(&db)
        .await
        .expect("Failed to fetch saved subscription count");

    assert_eq!(saved.count, Some(1));
}

#[sqlx::test]
async fn subscribe_returns_a_400_for_missing_form_data(db: PgPool) {
    let settings = Settings::test().expect("Unable to initialize configuration");

    let mut app = server::startup::app(settings, Some(db)).await;

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
