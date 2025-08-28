use actix_web::web::{self, get, post, resource, scope, ServiceConfig};

use crate::controllers::health::{check_health, not_found};
use crate::controllers::orders::{add_order, get_one_order, get_order_list, remove_order};
use crate::controllers::status::{check_user, save_user_test};
use crate::controllers::user::{fetch_all, register_user, user_login};

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("api/v1")
            .service(resource("/check_status").route(get().to(check_health)))
            .service(
                scope("/users")
                    .service(resource("/login").route(post().to(user_login)))
                    .service(resource("/register").route(post().to(register_user)))
                    .service(resource("/fetch_all").route(get().to(fetch_all))),
            )
            .service(
                scope("/orders")
                    .service(resource("create_order").route(post().to(add_order)))
                    .service(resource("/delete_order").route(get().to(remove_order)))
                    .service(resource("/get_one").route(get().to(get_one_order)))
                    .service(resource("order_list").route(get().to(get_order_list))),
            )
            .service(resource("/check_user_status").route(get().to(check_user)))
            .service(resource("/save_user_test").route(post().to(save_user_test))),
    );
    //catch all routes
    cfg.default_service(web::to(not_found));
}
