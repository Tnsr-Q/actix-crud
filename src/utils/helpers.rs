use std::env;

use actix_web::cookie::time::Duration;
use actix_web::cookie::Cookie;
use dotenv::dotenv;

use super::constants::COOKIE_NAME;

pub fn get_conn_url() -> String {
    dotenv().ok();

    let db_user = env::var("DB_USER").expect("Key not found!!");
    let db_pass = env::var("DB_PASS").expect("Key not found!!");
    let db_host = env::var("DB_HOST").expect("Key not found!!");
    let db_name = env::var("DB_NAME").expect("Key not found!!");
    let db_port = env::var("DB_PORT").expect("Key not found!!");

    format!(
        "postgresql://{}:{}@{}:{}/{}",
        db_user.trim(),
        db_pass.trim(),
        db_host.trim(),
        db_port.trim(),
        db_name.trim()
    )
}

pub fn build_auth_cookie(token: String) -> Cookie<'static> {
    Cookie::build(COOKIE_NAME, token)
        .http_only(true)
        // .same_site(SameSite::Strict)  // Commented out as in original register_user
        .path("/")
        .max_age(Duration::hours(2))
        .finish()
}
