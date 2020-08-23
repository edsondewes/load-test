use mongodb::bson;
use serde::Serialize;
use std::convert::Infallible;
use thiserror::Error;
use warp::{http::StatusCode, reply, Rejection, Reply};

#[derive(Error, Debug)]
pub enum Error {
  #[error("mongodb error: {0}")]
  MongoError(#[from] mongodb::error::Error),
  #[error("error during mongodb query: {0}")]
  MongoQueryError(mongodb::error::Error),
  #[error("could not access field in document: {0}")]
  MongoDataError(#[from] bson::document::ValueAccessError),
}

#[derive(Serialize)]
struct ErrorResponse {
  message: String,
}

impl warp::reject::Reject for Error {}

pub async fn handle_rejection(_err: Rejection) -> std::result::Result<Box<dyn Reply>, Infallible> {
  let json = reply::json(&ErrorResponse {
    message: String::from("Internal Server Error"),
  });

  Ok(Box::new(reply::with_status(
    json,
    StatusCode::INTERNAL_SERVER_ERROR,
  )))
}
