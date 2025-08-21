use actix_cors::Cors;
use actix_web::middleware::from_fn;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use env_logger;
use log::info;
use sqlx::postgres::PgPoolOptions;
use std::{env, io};

use self::middlewares::auth::authenticate_request;
use self::middlewares::logger::log_requests;
use self::utils::helpers::get_conn_url;
mod controllers;
mod middlewares;
mod repository;
mod routes;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv().ok();

    info!("Starting the server");
    let db_url = get_conn_url();

    let db_pool = PgPoolOptions::new()
        .connect(db_url.as_str())
        .await
        .map_err(|e| {
            info!("Error in connecting to the Database {:?}", e);
            io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to connect to database :: {:?}", e).as_str(),
            )
        })?;

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT")
        .unwrap_or_else(|_| "6002".to_string())
        .parse::<u16>()
        .unwrap();
    let allowed_origin =
        env::var("ALLOWED_ORIGIN").unwrap_or_else(|_| "127.0.0.1:3000".to_string());

    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(db_pool.clone()))
            .wrap(
                Cors::default()
                    .allowed_origin(&allowed_origin.as_str())
                    .send_wildcard()
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(from_fn(authenticate_request))
            .wrap(from_fn(log_requests))
            .configure(routes::init)
    })
    .bind((host.as_str(), port))?;

    info!("Server is running 🚀");

    server.run().await
}
