use std::env;
use std::sync::Arc;

use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::error::ErrorUnauthorized;
use actix_web::middleware::Next;
use actix_web::{Error, HttpMessage};
use dotenv::dotenv;
use jsonwebtoken::{decode, DecodingKey, Validation};
use log::{error, info};

use crate::utils::types::Claims;

pub async fn authenticate_request(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    dotenv().ok();
    let header_value = match req.headers().get("Authorization") {
        Some(value) => value.to_str().ok(),
        None => None,
    };

    if let Some(auth_header) = header_value {
        if auth_header.contains("Bearer ") {
            let token = auth_header.replace("Bearer ", "");
            let decoding_key = env::var("ENCODING_KEY").expect("Key not found!!!");
            match decode::<Claims>(
                &token,
                &DecodingKey::from_secret(decoding_key.as_ref()),
                &Validation::new(jsonwebtoken::Algorithm::HS256),
            ) {
                Ok(decoded) => {
                    req.extensions_mut().insert(Arc::new(decoded.claims.sub));
                }
                Err(e) => {
                    return Err(ErrorUnauthorized(format!("Invalid Request !!! {:?}", e)));
                }
            }
        } else {
            return Err(ErrorUnauthorized("Invalid token !!"));
        }
    }

    info!("Auth called!!");
    let response = match next.call(req).await {
        Ok(res) => {
            info!("success");
            res
        }
        Err(e) => {
            error!("Error occured!! {:?}", e);
            return Err(e);
        }
    };
    Ok(response)
}
