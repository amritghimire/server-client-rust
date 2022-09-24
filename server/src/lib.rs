pub mod configuration;
pub mod error;
pub mod routes;
pub mod startup;

use axum::extract::rejection::JsonRejection;
use axum::Json;
use sqlx::PgPool;

pub type JsonPayload<T> = Result<Json<T>, JsonRejection>;
pub type JsonResponse<T> = Result<Json<T>, error::AppError>;

pub struct State {
    pub database: PgPool,
}
