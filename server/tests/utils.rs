use axum::http::Request;
use axum::response::Response;
use axum::Router;
use hyper::Body;
use tower::{Service, ServiceExt};

use server::{configuration::Settings, startup};

use sqlx::{Connection, PgConnection};

pub async fn get_database() -> PgConnection {
    let settings = Settings::test().expect("Unable to initialize configuration");

    let database_string = settings.database.connection_string();

    println!("Connecting to database at {}", database_string);
    let connection = PgConnection::connect(&settings.database.connection_string())
        .await
        .expect("Failed to connect to database");
    connection
}

pub async fn run_with_app(request: Request<Body>) -> Response {
    let app = startup::app(None);
    let response = app.oneshot(request).await.unwrap();
    response
}

pub async fn run_request(app: &mut Router, request: Request<Body>) -> Response {
    let response = app.ready().await.unwrap().call(request).await.unwrap();
    response
}
