use actix_web::web::Data;
use sqlx::PgPool;
use std::error::Error;

use crate::utils::types::RegisterUser;

pub struct UserRepo;

impl UserRepo {
    pub async fn user_registration(
        payload: RegisterUser,
        pool: &Data<PgPool>,
        sec_hash: String,
    ) -> Result<i32, Box<dyn Error>> {
        sqlx::query!(
            r#"INSERT INTO users (user_name, sec, user_login, address) VALUES (?,?,?,?)"#,
            payload.user_name,
            sec_hash,
            payload.user_login,
            payload.address,
        )
        .execute(pool)
        .await?;
        Ok(1)
    }
}
