use actix_web::web::{Data, Json};
use actix_web::{HttpResponse, Responder};
use sqlx::PgPool;

use crate::repository::user_repo::UserRepo;
use crate::utils::jwt_impl::generate_jwt_token;
use crate::utils::types::RegisterUser;

use super::api_responses::ApiResponse;

pub async fn register_user(payload: Json<RegisterUser>, pool: Data<PgPool>) -> impl Responder {
    let payload = payload.into_inner();
    let user_res = match UserRepo::user_registration(
        payload,
        &pool,
        String::from("asdhjgadhjgasdhjgasjhdgas"),
    )
    .await
    {
        Ok(res) => res,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ApiResponse::<String> {
                status: 500,
                msg: format!("Error occured !! {:?}", e),
                results: None,
            });
        }
    };

    let token = match generate_jwt_token(user_res) {
        Ok(token) => token,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ApiResponse::<String> {
                status: 500,
                msg: format!("Error occured !! {:?}", e),
                results: None,
            });
        }
    };
    HttpResponse::Ok().json(ApiResponse {
        status: 200,
        msg: String::from("User registered & token generated"),
        results: Some(token),
    })
}
