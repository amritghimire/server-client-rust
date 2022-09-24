use crate::{JsonPayload, JsonResponse};

use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SubscribeUser {
    email: String,
    name: String,
}

pub async fn subscribe(payload: JsonPayload<SubscribeUser>) -> JsonResponse<SubscribeUser> {
    let Json(user) = payload?;
    Ok(user.into())
}
