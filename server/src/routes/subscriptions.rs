use crate::{JsonPayload, JsonResponse, State};
use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;

use axum::{routing::post, Extension, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SubscribeUser {
    email: String,
    name: String,
}

pub fn router() -> Router {
    Router::new().route("/api/subscriptions", post(subscribe))
}

#[tracing::instrument(name = "Adding a new subscriber", skip(state))]
pub async fn subscribe(
    payload: JsonPayload<SubscribeUser>,
    Extension(state): Extension<Arc<State>>,
) -> JsonResponse<SubscribeUser> {
    let Json(user) = payload?;
    tracing::info!(
        "Adding '{}' '{}' as a new subscriber",
        user.email,
        user.name
    );

    let result = sqlx::query!(
        r#"
      INSERT INTO subscriptions (id, email, name, subscribed_at)
      values ($1, $2, $3, $4)
      "#,
        Uuid::new_v4(),
        user.email,
        user.name,
        Utc::now()
    )
    .execute(&state.database)
    .await;
    match result {
        Ok(_) => {
            tracing::info!("New subscriber details has been saved");
            Ok(user.into())
        }
        Err(err) => match err {
            sqlx::Error::Database(dbe) if dbe.constraint() == Some("subscriptions_email_key") => {
                tracing::info!("The subscriber already exists in database");
                Ok(user.into())
            }
            _ => Err(err.into()),
        },
    }
}
