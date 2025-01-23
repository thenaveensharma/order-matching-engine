use crate::matching_engine::types::decimal::Price;
use chrono::NaiveDateTime;
use diesel::prelude::Queryable;
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

#[derive(Debug, Serialize, Deserialize, Queryable)]
#[diesel(table_name = orders)]
pub struct Order {
    pub id: i32,
    pub user_id: uuid::Uuid,
    pub token_id: i32,
    pub order_type: String,
    pub price: Price,
    pub amount: Price,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Order {
    pub fn from_row(row: &Row) -> Self {
        Order {
            id: row.get("id"),
            user_id: row.get("user_id"),
            token_id: row.get("token_id"),
            order_type: row.get("order_type"),
            price: row.get("price"),
            amount: row.get("amount"),
            status: row.get("status"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}
