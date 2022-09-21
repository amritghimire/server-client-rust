use crate::StatusCode;
use axum::extract::rejection::JsonRejection;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use std::error::Error;

pub enum AppError {
    JsonParsing(JsonRejection),
}

impl From<JsonRejection> for AppError {
    fn from(inner: JsonRejection) -> Self {
        AppError::JsonParsing(inner)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::JsonParsing(err) => handle_json_error(err),
        };

        let body = Json(json!({ "error": error_message }));

        (status, body).into_response()
    }
}

pub fn handle_json_error(err: JsonRejection) -> (StatusCode, String) {
    match err {
        JsonRejection::JsonDataError(err) => serde_json_error_response(err),
        JsonRejection::JsonSyntaxError(err) => serde_json_error_response(err),
        // handle other rejections from the `Json` extractor
        JsonRejection::MissingJsonContentType(_) => (
            StatusCode::BAD_REQUEST,
            "Missing `Content-Type: application/json` header".to_string(),
        ),
        JsonRejection::BytesRejection(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to buffer request body".to_string(),
        ),
        // we must provide a catch-all case since `JsonRejection` is marked
        // `#[non_exhaustive]`
        _ => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Unknown error".to_string(),
        ),
    }
}

// attempt to extract the inner `serde_json::Error`, if that succeeds we can
// provide a more specific error
fn serde_json_error_response<E>(err: E) -> (StatusCode, String)
where
    E: Error + 'static,
{
    if let Some(serde_json_err) = find_error_source::<serde_json::Error>(&err) {
        (StatusCode::BAD_REQUEST, serde_json_err.to_string())
    } else {
        (StatusCode::BAD_REQUEST, "Unknown error".to_string())
    }
}

// attempt to downcast `err` into a `T` and if that fails recursively try and
// downcast `err`'s source
fn find_error_source<'a, T>(err: &'a (dyn Error + 'static)) -> Option<&'a T>
where
    T: Error + 'static,
{
    if let Some(err) = err.downcast_ref::<T>() {
        Some(err)
    } else if let Some(source) = err.source() {
        find_error_source(source)
    } else {
        None
    }
}
