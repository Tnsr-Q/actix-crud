use std::error::Error;

use actix_web::web::Data;
use sqlx::postgres::PgRow;
use sqlx::{PgPool, Row};

use crate::utils::types::OrderDetails;

pub struct OrderRepo;

impl OrderRepo {
    pub async fn get_one_order_detail(
        order_id: &i32,
        pool: &Data<PgPool>,
    ) -> Result<OrderDetails, Box<dyn Error>> {
        let row: PgRow = sqlx::query(
            r#"SELECT order_id, description, created_at FROM orders WHERE order_id = $1 AND is_active = $2"#,
        )
        .bind(order_id)
        .bind(true)
        .fetch_one(pool.as_ref())
        .await?;
        let order_details = OrderDetails {
            order_id: row.get("order_id"),
            description: row.get("description"),
            created_at: row.get("created_at"),
        };
        Ok(order_details)
    }

    pub async fn fetch_orders(pool: &Data<PgPool>) -> Result<Vec<OrderDetails>, Box<dyn Error>> {
        let rows: Vec<PgRow> = sqlx::query(r#"SELECT * FROM orders WHERE is_active = $1"#)
            .bind(true)
            .fetch_all(pool.as_ref())
            .await?;
        let mut order_list = Vec::new();
        for row in rows {
            order_list.push(OrderDetails {
                order_id: row.get("order_id"),
                description: row.get("description"),
                created_at: row.get("created_at"),
            });
        }
        Ok(order_list)
    }

    pub async fn deactivate_order(
        order_id: &i32,
        pool: &Data<PgPool>,
    ) -> Result<u64, Box<dyn Error>> {
        let result = sqlx::query(r#"UPDATE orders SET is_active = $1 WHERE order_id = $2"#)
            .bind(false)
            .bind(order_id)
            .execute(pool.as_ref())
            .await?;
        Ok(result.rows_affected())
    }

    pub async fn create_order(desc: String, pool: &Data<PgPool>) -> Result<u64, Box<dyn Error>> {
        let result = sqlx::query(r#"INSERT INTO orders (description) VALUES ($1)"#)
            .bind(desc)
            .execute(pool.as_ref())
            .await?;

        Ok(result.rows_affected())
    }
}
