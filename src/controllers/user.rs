use actix_web::cookie::time::Duration;
use actix_web::cookie::{Cookie, SameSite}; main
use actix_web::web::{Data, Json};
use actix_web::{HttpResponse, Responder};
use sqlx::PgPool;

use crate::repository::user_repo::UserRepo;
use crate::utils::helpers::build_auth_cookie;
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

/// Register a new user, issue a JWT, and store it in a secure HTTP-only cookie.
///
/// On success this handler sets a cookie named `"OKIJ"` containing `"Bearer <token>"` with
/// SameSite=None, Secure, HttpOnly, Path="/" and a 2-hour max age, and returns HTTP 200 with
/// an `ApiResponse` whose `msg` is `"User registered & token generated"`. If user registration
/// or token generation fails the handler responds with HTTP 500 and an error message.
///
/// # Examples
///
/// ```no_run
/// use actix_web::test;
/// use actix_web::web::Json;
///
/// // Build a RegisterUser payload and a PgPool `Data` wrapper before calling.
/// let payload = Json(RegisterUser { /* fill required fields */ });
/// let pool = /* Data<PgPool> instance */;
///
/// // Call the handler (in a test runtime)
/// let resp = test::block_on(register_user(payload, pool));
/// assert!(resp.status().is_success());
/// ```
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
        Ok(token) => "Bearer ".to_string() + &token,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ApiResponse::<String> {
                status: 500,
                msg: format!("Error occured !! {:?}", e),
                results: None,
            });
        }
    };


    let cookie = build_auth_cookie(token.clone());

    let cookie = Cookie::build("OKIJ", &token)
        .http_only(true)
        .same_site(SameSite::None)
        .secure(true)
        .path("/")
        .max_age(Duration::hours(2))
        .finish(); main

    HttpResponse::Ok()
        .cookie(cookie)
        .json(ApiResponse::<String> {
            status: 200,
            msg: String::from("User registered & token generated"),
            results: None,
        })
}

/// Authenticate a user and, on success, set a JWT in an HTTP-only secure cookie.
///
/// On success: returns HTTP 200 with a JSON ApiResponse and a cookie named `"OKIJ"` containing the JWT.
/// The cookie is HttpOnly, SameSite=None, Secure, Path="/", and expires after 2 hours.
/// If credentials are invalid: returns HTTP 401 with an unauthorized ApiResponse.
/// On repository or validation errors: returns HTTP 500 with an error ApiResponse.
///
/// # Examples
///
/// ```no_run
/// use actix_web::{test, web::Json};
/// // Construct a `UserLogin` payload and call the handler in an integration-style test.
/// // On successful credentials the response will include a cookie named "OKIJ".
/// let req_payload = UserLogin { /* fields */ };
/// let resp = test::block_on(user_login(Json(req_payload), /* pool */));
/// // inspect resp for status and cookie
/// ```
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
      
        let token = match generate_jwt_token(user_details.id) {
            Ok(token) => "Bearer ".to_string() + &token,
            Err(e) => {
                return HttpResponse::InternalServerError().json(ApiResponse::<String> {
                    status: 500,
                    msg: format!("Error occured !! {:?}", e),
                    results: None,
                });
            }
        };
        
        let cookie = build_auth_cookie(token.clone());
        
        HttpResponse::Ok().cookie(cookie).json(ApiResponse {
            status: 200,
            msg: "User Loggedin !!".to_string(),
            results: Some(token),
        })

        let token = generate_jwt_token(user_details.id).unwrap();
        let cookie = Cookie::build("OKIJ", &token)
            .http_only(true)
            .same_site(SameSite::None)
            .secure(true)
            .path("/")
            .max_age(Duration::hours(2))
            .finish();

        HttpResponse::Ok()
            .cookie(cookie)
            .json(ApiResponse::<String> {
                status: 200,
                msg: format!("User Loggedin !!"),
                results: None,
            })  main
    } else {
        HttpResponse::Unauthorized().json(ApiResponse::<String> {
            status: 401,
            msg: format!("Uauthorized Access !!!"),
            results: None,
        })
    }
}
