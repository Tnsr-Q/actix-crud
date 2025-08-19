use actix_web::web::{self, get, post, resource, scope, ServiceConfig};

use crate::controllers::health::{check_health, not_found};
use crate::controllers::user::{check_user, save_user};

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("api/v1")
            .service(resource("/check_status").route(get().to(check_health)))
            .service(resource("/check_user").route(get().to(check_user)))
            .service(resource("/save_user").route(post().to(save_user))),
    );
    //catch all routes
    cfg.default_service(web::to(not_found));
}
