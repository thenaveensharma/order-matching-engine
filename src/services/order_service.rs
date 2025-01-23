use rust_decimal::prelude::Zero;

use crate::db::pool::DbPool;
use crate::domain::order::Order;
use crate::matching_engine::types::decimal::Price;
use crate::repository::order_repository::OrderRepository;

pub struct OrderService;

impl OrderService {
    pub async fn create_order(pool: &DbPool, new_order: &Order) -> Result<Order, String> {
        if new_order.amount <= Price::from(rust_decimal::Decimal::zero()) {
            return Err("Order amount must be greater than zero".to_string());
        }
        if new_order.price <= Price::from(rust_decimal::Decimal::zero()) {
            return Err("Order price must be greater than zero".to_string());
        }

        Ok(OrderRepository::create_order(pool, new_order)
            .await
            .unwrap())
    }

    pub async fn get_order(pool: &DbPool, order_id: i32) -> Result<Order, String> {
        OrderRepository::find_by_id(pool, order_id)
            .await
            .map_err(|e| e.to_string())
    }
}
