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
        // let response = sqlx::query(
        //     r#"INSERT INTO users (user_name, sec, user_login, address) VALUES ($1,$2,$3,$4)"#,
        // )
        // .bind(payload.user_name)
        // .bind(sec_hash)
        // .bind(payload.user_login)
        // .bind(payload.address)
        // .execute(pool.as_ref())
        // .await?;
        // //TODO::
        // println!("res :: {:?}", response);
        Ok(1)
    }
}
