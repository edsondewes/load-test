use crate::{auth::decode_token, repository::DB, WebResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use warp::{reject, reply::json, Reply};

#[derive(Deserialize)]
pub struct CreateMessageRequest {
  to: String,
  text: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MessageViewModel {
  message_id: String,
  app_name: String,
  text: String,
  to: String,
  from: String,
  status: String,
  created_at: DateTime<Utc>,
  updated_at: DateTime<Utc>,
}

pub async fn post_message_handler(
  token: String,
  body: CreateMessageRequest,
  db: DB,
) -> WebResult<impl Reply> {
  let jwt = decode_token(&token).unwrap();
  let app = db
    .find_app_by_name(&jwt.app_name)
    .await
    .map_err(|e| reject::custom(e))?;

  let message = db
    .save_message(&app, &body.to, &body.text)
    .await
    .map_err(|e| reject::custom(e))?;

  db.set_interaction_last_message(&message)
    .await
    .map_err(|e| reject::custom(e))?;

  let result = MessageViewModel {
    message_id: message.id.to_hex(),
    app_name: message.app_name,
    text: message.text,
    to: message.to,
    from: message.from,
    status: message.status,
    created_at: message.created_at,
    updated_at: message.updated_at,
  };

  Ok(json(&result))
}
