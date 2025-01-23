use crate::db::pool::DbPool;
use crate::domain::order::Order;
use crate::errors::custom_error::OrderError;

pub struct OrderRepository;

impl OrderRepository {
    pub async fn create_order(pool: &DbPool, new_order: &Order) -> Result<Order, OrderError> {
        let client = pool.get_connection().await?;
        let row = client
            .query_one(
                "INSERT INTO orders (user_id, token_id, order_type, price, amount, status) 
                 VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
                &[
                    &new_order.user_id,
                    &new_order.token_id,
                    &new_order.order_type,
                    &new_order.price,
                    &new_order.amount,
                    &new_order.status,
                ],
            )
            .await?;
        Ok(Order::from_row(&row))
    }

    pub async fn find_by_id(pool: &DbPool, order_id: i32) -> Result<Order, OrderError> {
        let client = pool.get_connection().await?;
        let row = client
            .query_one("SELECT * FROM orders WHERE id = $1", &[&order_id])
            .await?;
        Ok(Order::from_row(&row))
    }

    pub async fn update_order_status(
        pool: &DbPool,
        order_id: i32,
        new_status: &str,
    ) -> Result<Order, OrderError> {
        let client = pool.get_connection().await?;
        let row = client
            .query_one(
                "UPDATE orders SET status = $1 WHERE id = $2 RETURNING *",
                &[&new_status, &order_id],
            )
            .await?;
        Ok(Order::from_row(&row))
    }

    pub async fn delete_order(pool: &DbPool, order_id: i32) -> Result<usize, OrderError> {
        let client = pool.get_connection().await?;
        let result = client
            .execute("DELETE FROM orders WHERE id = $1", &[&order_id])
            .await?;
        Ok(result as usize)
    }
}
