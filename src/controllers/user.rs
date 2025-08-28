use actix_web::web::{Data, Json};
use actix_web::{HttpResponse, Responder};
use sqlx::PgPool;

use crate::repository::user_repo::UserRepo;
use crate::utils::jwt_impl::{generate_jwt_token, get_hash, validate_hash};
use crate::utils::types::{RegisterUser, UserLogin};

use super::api_responses::ApiResponse;

pub async fn fetch_all(pool: Data<PgPool>) -> impl Responder {
    let user_list = match UserRepo::fetch_users_list(&pool).await {
        Ok(res) => res,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ApiResponse::<String> {
                status: 500,
                msg: format!("Error occured!! {:?}", e),
                results: None,
            });
        }
    };
    HttpResponse::Ok().json(ApiResponse {
        status: 200,
        msg: format!("Users fetched !!"),
        results: Some(user_list),
    })
}

pub async fn register_user(payload: Json<RegisterUser>, pool: Data<PgPool>) -> impl Responder {
    let payload = payload.into_inner();
    let sec = &payload.sec;
    let hash = get_hash(sec).unwrap();
    let user_res = match UserRepo::user_registration(payload, &pool, hash).await {
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

pub async fn user_login(payload: Json<UserLogin>, pool: Data<PgPool>) -> impl Responder {
    let payload = &payload.into_inner();
    let user_details = match UserRepo::fetch_one_user(payload, &pool).await {
        Ok(res) => res,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ApiResponse::<String> {
                status: 500,
                msg: format!("Error occured !! {:?}", e),
                results: None,
            });
        }
    };
    let is_valid = match validate_hash(user_details.sec, &payload.sec) {
        Ok(valid) => valid,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ApiResponse::<String> {
                status: 500,
                msg: format!("Error occured !! {:?}", e),
                results: None,
            });
        }
    };
    if is_valid {
        let token = generate_jwt_token(user_details.id).unwrap();
        HttpResponse::Ok().json(ApiResponse {
            status: 200,
            msg: format!("User Loggedin !!"),
            results: Some(token),
        })
    } else {
        HttpResponse::Unauthorized().json(ApiResponse::<String> {
            status: 401,
            msg: format!("Uauthorized Access !!!"),
            results: None,
        })
    }
}
