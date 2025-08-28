use actix_web::web::{Data, Json, Query};
use actix_web::{HttpResponse, Responder};
use sqlx::PgPool;

use crate::repository::order_repo::OrderRepo;
use crate::utils::types::{Order, SingleOrder};

use super::api_responses::ApiResponse;

pub async fn get_one_order(query: Query<SingleOrder>, pool: Data<PgPool>) -> impl Responder {
    let order_id = &query.order_id;
    let res = match OrderRepo::get_one_order_detail(&order_id, &pool).await {
        Ok(res) => res,
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
        msg: "Details Fetched !!".to_string(),
        results: Some(res),
    })
}

pub async fn get_order_list(pool: Data<PgPool>) -> impl Responder {
    let order_list = match OrderRepo::fetch_orders(&pool).await {
        Ok(res) => res,
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
        msg: format!("Order List fetched !! {} Records", order_list.len()),
        results: Some(order_list),
    })
}

pub async fn remove_order(req: Query<SingleOrder>, pool: Data<PgPool>) -> impl Responder {
    let order_id = req.order_id;
    let res = match OrderRepo::deactivate_order(&order_id, &pool).await {
        Ok(res) => res,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ApiResponse::<String> {
                status: 500,
                msg: format!("Error occured !! {:?}", e),
                results: None,
            });
        }
    };
    HttpResponse::Ok().json(ApiResponse::<String> {
        status: 200,
        msg: format!(
            "OK Rows effected {}!!!\n Order ID :: {:?}, Removed!!!",
            res, order_id
        ),
        results: None,
    })
}

pub async fn add_order(payload: Json<Order>, pool: Data<PgPool>) -> impl Responder {
    let body = payload.into_inner();
    let description = body.description;
    let res = match OrderRepo::create_order(description, &pool).await {
        Ok(res) => res,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ApiResponse::<String> {
                status: 500,
                msg: format!("Error occured !! {:?}", e),
                results: None,
            });
        }
    };
    HttpResponse::Ok().json(ApiResponse::<String> {
        status: 200,
        msg: format!("Order created !!\nRows Effected :: {}", res),
        results: None,
    })
}
