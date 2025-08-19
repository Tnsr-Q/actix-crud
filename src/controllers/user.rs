use std::sync::Arc;

use actix_web::web::{Data, Json};
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder};
use sqlx::PgPool;

use crate::utils::types::{UserDetail, UserInfo, UserPayload};

use super::api_responses::ApiResponse;

pub async fn check_user(req: HttpRequest, _pool: Data<PgPool>) -> impl Responder {
    let extentions = req.extensions();
    if let Some(user_details) = extentions.get::<Arc<UserInfo>>() {
        HttpResponse::Ok().json(ApiResponse {
            status: 200,
            msg: format!("User info fetched!!"),
            results: Some(user_details),
        })
    } else {
        HttpResponse::Unauthorized().json(ApiResponse::<String> {
            status: 400,
            msg: format!("Unauthorized Access!!"),
            results: None,
        })
    }
}

pub async fn save_user(
    req: HttpRequest,
    _pool: Data<PgPool>,
    payload: Json<UserPayload>,
) -> impl Responder {
    let payload = payload.into_inner();
    let extentions = req.extensions();

    if let Some(user_details) = extentions.get::<Arc<UserInfo>>() {
        let user_item = UserDetail {
            user_info: UserInfo {
                user_id: user_details.user_id,
            },
            user_payload: payload,
        };
        HttpResponse::Ok().json(ApiResponse {
            status: 200,
            msg: format!("All is well !!"),
            results: Some(user_item),
        })
    } else {
        HttpResponse::Unauthorized().json(ApiResponse::<String> {
            status: 400,
            msg: format!("Unauthorized Access!!"),
            results: None,
        })
    }
}
