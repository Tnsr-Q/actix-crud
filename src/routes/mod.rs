use actix_web::web::{self, get, resource, scope, ServiceConfig};

use crate::controllers::health::{check_health, not_found};

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(scope("api/v1").service(resource("/check_status").route(get().to(check_health))));
    //catch all routes
    cfg.default_service(web::to(not_found));
}
