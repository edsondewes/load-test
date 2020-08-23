use jsonwebtoken::errors::Error;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

const SECRET: &str = "my super secret test key";
const TOKEN_PREFIX: &str = "Bearer ";

#[derive(Deserialize, Serialize, Debug)]
pub struct Claims {
  pub app_name: String,
  exp: usize,
}

pub fn decode_token(token: &str) -> Result<Claims, Error> {
  decode::<Claims>(
    token.trim_start_matches(TOKEN_PREFIX),
    &DecodingKey::from_secret(SECRET.as_ref()),
    &Validation::default(),
  )
  .map(|token_data| token_data.claims)
}
