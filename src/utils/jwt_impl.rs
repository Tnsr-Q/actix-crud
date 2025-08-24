use std::env;

use super::types::Claims;
use bcrypt::{hash, BcryptError, DEFAULT_COST};
use chrono::{Duration, Utc};
use dotenv::dotenv;
use jsonwebtoken::errors::Error;
use jsonwebtoken::{encode, EncodingKey, Header};
use log::info;

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

pub fn get_hash(pass: &String) -> Result<String, BcryptError> {
    let hash = match hash(pass, DEFAULT_COST) {
        Ok(hash) => hash,
        Err(e) => {
            info!("Error occured!! {:?}", e);
            return Err(e);
        }
    };
    Ok(hash)
}
