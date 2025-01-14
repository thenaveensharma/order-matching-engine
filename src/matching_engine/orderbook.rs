#![allow(dead_code)]
use super::types::order::{OrderRecord, OrderSide, OrderStatus};
use chrono::Utc;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use std::collections::HashMap;
use uuid::Uuid;
#[derive(Debug)]
pub struct Order {
    side: OrderSide,
    size: f64,
}

impl Order {
    fn new(side: OrderSide, size: f64) -> Order {
        Order { side, size }
    }
}
#[derive(Debug)]
pub struct OrderBook {
    asks: HashMap<Decimal, Limit>,
    bids: HashMap<Decimal, Limit>,
}

impl OrderBook {
    pub fn new() -> OrderBook {
        OrderBook {
            asks: HashMap::new(),
            bids: HashMap::new(),
        }
    }

    pub async fn add_limit_order_with_db(
        &mut self,
        price: Decimal,
        order: Order,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let order_record = OrderRecord {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            price,
            size: Decimal::from_f64(order.size).unwrap(),
            remaining_size: Decimal::from_f64(order.size).unwrap(),
            side: order.side,
            status: OrderStatus::New,
        };

        //save to database
        // db.save_order(&order_record).await?;

        //Add to in-memory orderbook
        self.add_limit_order(price, order_record);
        Ok(())
    }
    pub fn fill_market_order(&mut self, market_order: &mut OrderRecord) {
        // If it's a sell market order, look at bids
        // If it's a buy market order, look at asks
        let limits = match market_order.side {
            OrderSide::Ask => self.bid_limits(),
            OrderSide::Bid => self.ask_limits(),
        };

        for limit_order in limits {
            limit_order.fill_order(market_order);

            if market_order.is_filled() {
                break;
            }
        }
    }

    pub fn ask_limits(&mut self) -> Vec<&mut Limit> {
        let mut limits = self.asks.values_mut().collect::<Vec<&mut Limit>>();
        limits.sort_by(|a, b| a.price.cmp(&b.price));
        limits
    }

    pub fn bid_limits(&mut self) -> Vec<&mut Limit> {
        let mut limits = self.bids.values_mut().collect::<Vec<&mut Limit>>();
        limits.sort_by(|a, b| b.price.cmp(&a.price));
        limits
    }
    pub fn add_limit_order(&mut self, price: Decimal, order: OrderRecord) {
        match order.side {
            OrderSide::Bid => match self.bids.get_mut(&price) {
                Some(limit) => {
                    limit.add_order(order);
                }
                None => {
                    let mut limit = Limit::new(price);
                    limit.add_order(order);
                    self.bids.insert(price, limit);
                }
            },
            OrderSide::Ask => match self.asks.get_mut(&price) {
                Some(limit) => {
                    limit.add_order(order);
                }
                None => {
                    let mut limit = Limit::new(price);
                    limit.add_order(order);
                    self.asks.insert(price, limit);
                }
            },
        }
    }
}

#[derive(Debug)]
pub struct Limit {
    price: Decimal,
    orders: Vec<OrderRecord>,
}

impl Limit {
    fn new(price: Decimal) -> Limit {
        Limit {
            price,
            orders: Vec::new(),
        }
    }
    fn add_order(&mut self, order: OrderRecord) {
        self.orders.push(order);
    }

    fn total_volume(&self) -> Decimal {
        self.orders.iter().map(|order| order.size).sum()
    }

    fn fill_order(&mut self, market_order: &mut OrderRecord) {
        for limit_order in self.orders.iter_mut() {
            match market_order.size >= limit_order.size {
                true => {
                    market_order.size -= limit_order.size;
                    limit_order.size = dec!(0.0)
                }

                false => {
                    limit_order.size -= market_order.size;
                    market_order.size = dec!(0.0)
                }
            }

            if market_order.is_filled() {
                break;
            }
        }
    }
}

impl OrderRecord {
    pub fn new(side: OrderSide, size: Decimal) -> OrderRecord {
        OrderRecord {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            side,
            size,
            price: dec!(100),
            remaining_size: dec!(0.0),
            status: OrderStatus::New,
        }
    }

    pub fn is_filled(&self) -> bool {
        self.size == dec!(0.0)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[tokio::test]
    async fn orderbook_should_fill_market_buy_order_complete() {
        let mut orderbook = OrderBook::new();
        orderbook.add_limit_order(dec!(300), OrderRecord::new(OrderSide::Ask, dec!(20.0)));
        orderbook.add_limit_order(dec!(200), OrderRecord::new(OrderSide::Ask, dec!(30.0)));
        orderbook.add_limit_order(dec!(100), OrderRecord::new(OrderSide::Ask, dec!(10.0)));
        orderbook.add_limit_order(dec!(600), OrderRecord::new(OrderSide::Ask, dec!(40.0)));
        orderbook.add_limit_order(dec!(400), OrderRecord::new(OrderSide::Ask, dec!(50.0)));

        let mut market_order = OrderRecord::new(OrderSide::Bid, dec!(10.0));

        orderbook.fill_market_order(&mut market_order);

        let ask_limits = orderbook.ask_limits();
        let matched_limit = ask_limits.get(0).unwrap();
        assert_eq!(matched_limit.price, dec!(100.0));
        assert_eq!(market_order.is_filled(), true);

        let matched_order = matched_limit.orders.get(0).unwrap();
        assert_eq!(matched_order.is_filled(), true);
    }
    #[tokio::test]
    async fn total_volume() {
        let price = dec!(99.99);
        let mut limit = Limit::new(price);
        assert_eq!(limit.total_volume(), dec!(0.0));

        let limit_buy_order_1 = OrderRecord::new(OrderSide::Bid, dec!(200.0));
        let limit_buy_order_2 = OrderRecord::new(OrderSide::Bid, dec!(300.0));

        limit.add_order(limit_buy_order_1);
        assert_eq!(limit.total_volume(), dec!(200.0));

        limit.add_order(limit_buy_order_2);
        assert_eq!(limit.total_volume(), dec!(500.0));

        let mut market_sell_order = OrderRecord::new(OrderSide::Ask, dec!(100.0));

        limit.fill_order(&mut market_sell_order);

        assert_eq!(limit.total_volume(), dec!(400.0));
    }

    #[tokio::test]
    async fn fill_limit_order_single() {
        let price = dec!(99.99);
        let mut limit = Limit::new(price);
        let limit_buy_order = OrderRecord::new(OrderSide::Bid, dec!(100.0));
        limit.add_order(limit_buy_order);

        let mut market_sell_order = OrderRecord::new(OrderSide::Ask, dec!(99.0));

        limit.fill_order(&mut market_sell_order);

        assert_eq!(market_sell_order.is_filled(), true);
        assert_eq!(limit.orders.get(0).unwrap().size, dec!(1.0));
    }

    #[tokio::test]
    async fn fill_limit_order_multi() {
        let price = dec!(99.99);
        let mut limit = Limit::new(price);
        let limit_buy_order_1 = OrderRecord::new(OrderSide::Bid, dec!(100.0));
        let limit_buy_order_2 = OrderRecord::new(OrderSide::Bid, dec!(100.0));
        limit.add_order(limit_buy_order_1);
        limit.add_order(limit_buy_order_2);

        let mut market_sell_order = OrderRecord::new(OrderSide::Ask, dec!(199.0));

        limit.fill_order(&mut market_sell_order);

        assert_eq!(market_sell_order.is_filled(), true);
        assert_eq!(limit.orders.get(0).unwrap().is_filled(), true);
        assert_eq!(limit.orders.get(1).unwrap().is_filled(), false);
        assert_eq!(limit.orders.get(1).unwrap().size, dec!(1.0));
    }
}
