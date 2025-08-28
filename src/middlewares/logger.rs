use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::middleware::Next;
use actix_web::Error;
use log::{error, info};

pub async fn log_requests(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    info!(
        "::Incoming Request::\n Path:: {:?}\n Method:: {:?}",
        req.path(),
        req.method()
    );

    let response = match next.call(req).await {
        Ok(res) => {
            info!("::Request completed Successfully::\n",);
            res
        }
        Err(e) => {
            error!("::Error occured:: {:?}\n", e);
            return Err(e);
        }
    };

    Ok(response)
}
