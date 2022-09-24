pub mod error;
pub mod routes;
pub mod startup;
pub mod configuration;

use axum::extract::rejection::JsonRejection;
use axum::Json;

pub type JsonPayload<T> = Result<Json<T>, JsonRejection>;
pub type JsonResponse<T> = Result<Json<T>, error::AppError>;
