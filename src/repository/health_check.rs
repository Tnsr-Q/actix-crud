use std::error::Error;

use actix_web::web::Data;
use sqlx::postgres::PgRow;
use sqlx::PgPool;

pub struct HealthCheckRepo;

impl HealthCheckRepo {
    pub async fn check_status(pool: &Data<PgPool>) -> Result<String, Box<dyn Error>> {
        let res: PgRow = sqlx::query("SELECT now() as current_time")
            .fetch_one(pool.as_ref())
            .await?;
        println!("{:?}", res);
        Ok(String::from("OK"))
    }
}
