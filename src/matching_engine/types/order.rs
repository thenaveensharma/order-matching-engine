use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub enum OrderSide {
    Bid,
    Ask,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OrderStatus {
    New,
    Filled,
    PartiallyFilled,
    Cancelled,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderRecord {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub price: rust_decimal::Decimal,
    pub size: rust_decimal::Decimal,
    pub remaining_size: rust_decimal::Decimal,
    pub side: OrderSide,
    pub status: OrderStatus,
}
