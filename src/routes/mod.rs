use actix_web::web::{self, get, post, resource, scope, ServiceConfig};

use crate::controllers::health::{check_health, not_found};
use crate::controllers::status::{check_user, save_user_test};
use crate::controllers::user::register_user;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("api/v1")
            .service(resource("/check_status").route(get().to(check_health)))
            .service(scope("/users").service(resource("/register").route(post().to(register_user))))
            .service(resource("/check_user_status").route(get().to(check_user)))
            .service(resource("/save_user_test").route(post().to(save_user_test))),
    );
    //catch all routes
    cfg.default_service(web::to(not_found));
}
