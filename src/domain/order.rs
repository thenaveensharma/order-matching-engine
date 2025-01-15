use crate::db::schema::orders;
use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize, AsChangeset)]
#[table_name = "orders"]
pub struct Order {
    pub id: i32,
    pub user_id: uuid::Uuid,
    pub token_id: i32,
    pub order_type: String,
    pub price: Decimal,
    pub amount: Decimal,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
