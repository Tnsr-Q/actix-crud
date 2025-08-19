use std::sync::Arc;

use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::middleware::Next;
use actix_web::{Error, HttpMessage};
use log::{error, info};

use crate::utils::types::UserInfo;

pub async fn authenticate_request(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    //fetch req headers
    //auth_token
    //Bearer
    //JWT

    let user = UserInfo { user_id: 1 };
    req.extensions_mut().insert(Arc::new(user));
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
