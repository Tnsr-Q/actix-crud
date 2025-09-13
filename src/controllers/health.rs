use actix_web::web::Data;
use actix_web::{HttpResponse, Responder};
use sqlx::PgPool;

use crate::repository::health_check::HealthCheckRepo;

use super::api_responses::ApiResponse;

pub async fn check_health(pool: Data<PgPool>) -> impl Responder {
    let status = match HealthCheckRepo::check_status(&pool).await {
        Ok(status) => status,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ApiResponse::<String> {
                status: 500,
                msg: format!("Error occured :: {:?}", e),
                results: None,
            })
        }
    };
    HttpResponse::Ok().json(ApiResponse {
        status: 200,
        msg: String::from("Server is Healthy and Running !!"),
        results: Some(status),
    })
}

pub async fn not_found() -> impl Responder {
    HttpResponse::NotFound().json(ApiResponse::<String> {
        status: 404,
        msg: String::from("Route not found!!"),
        results: None,
    })
}
