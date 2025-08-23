use std::env;

use super::types::Claims;
use chrono::{Duration, Utc};
use dotenv::dotenv;
use jsonwebtoken::errors::Error;
use jsonwebtoken::{encode, EncodingKey, Header};

pub fn generate_jwt_token(user_id: i32) -> Result<String, Error> {
    dotenv().ok();
    let now = Utc::now();
    let exp = (now + Duration::days(2)).timestamp();
    let encoding_key = env::var("ENCODING_KEY").expect("Key not found!!");
    let claims = Claims {
        sub: user_id,
        exp,
        aud: String::from("Vipin"),
        iss: String::from("vipin"),
        iat: Utc::now().timestamp(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(encoding_key.as_ref()),
    );
    token
}
