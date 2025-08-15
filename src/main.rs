use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use env_logger;
use log::info;
use sqlx::postgres::PgPoolOptions;
use std::{env, io};
mod controllers;
mod repository;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv().ok();

    info!("Starting the server");
    let db_url = env::var("DB_URL").expect("DB URL not found on the .env file");

    let db_pool = PgPoolOptions::new()
        .connect(db_url.as_str())
        .await
        .map_err(|e| {
            // info!("Error in connecting to the Database {:?}", e);
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
            .configure(routes::init)
    })
    .bind((host.as_str(), port))?;

    server.run().await
}
