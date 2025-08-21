use std::env;

use dotenv::dotenv;

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
