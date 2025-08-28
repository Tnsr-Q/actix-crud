use actix_web::web::Data;
use sqlx::postgres::PgRow;
use sqlx::{PgPool, Row};
use std::error::Error;

use crate::utils::types::{RegisterUser, UserDetails, UserLogin, Users};

pub struct UserRepo;

impl UserRepo {
    pub async fn fetch_users_list(pool: &Data<PgPool>) -> Result<Vec<Users>, Box<dyn Error>> {
        let rows: Vec<PgRow> = sqlx::query("SELECT user_login FROM app_users")
            .fetch_all(pool.as_ref())
            .await?;
        let mut users_list = vec![];
        for row in rows {
            users_list.push(Users {
                user_login: row.get("user_login"),
            });
        }
        Ok(users_list)
    }

    pub async fn user_registration(
        payload: RegisterUser,
        pool: &Data<PgPool>,
        sec_hash: String,
    ) -> Result<i32, Box<dyn Error>> {
        let result = sqlx::query(
            r#"INSERT INTO app_users (user_name, sec, user_login, address) VALUES ($1,$2,$3,$4)"#,
        )
        .bind(&payload.user_name)
        .bind(sec_hash)
        .bind(&payload.user_login)
        .bind(&payload.address)
        .execute(pool.as_ref())
        .await?;

        if result.rows_affected() > 0 {
            let user_row: i32 =
                sqlx::query_scalar(r#"SELECT id from app_users WHERE user_login = $1"#)
                    .bind(&payload.user_login)
                    .fetch_one(pool.as_ref())
                    .await?;

            return Ok(user_row);
        }
        Err("Record not inserted!!".into())
    }

    pub async fn fetch_one_user(
        payload: &UserLogin,
        pool: &Data<PgPool>,
    ) -> Result<UserDetails, Box<dyn Error>> {
        let row = sqlx::query("SELECT id, sec from app_users WHERE user_login = $1")
            .bind(&payload.user_login)
            .fetch_one(pool.as_ref())
            .await?;
        let user_details = UserDetails {
            id: row.get("id"),
            sec: row.get("sec"),
        };
        Ok(user_details)
    }
}
