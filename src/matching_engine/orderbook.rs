use std::collections::HashMap;

#[derive(Debug)]
pub enum BidOrAsk {
    Bid,
    Ask,
}

#[derive(Debug)]
pub struct OrderBook {
    asks: HashMap<Price, Limit>,
    bids: HashMap<Price, Limit>,
}

impl OrderBook {
    pub fn new() -> OrderBook {
        OrderBook {
            asks: HashMap::new(),
            bids: HashMap::new(),
        }
    }
    pub fn fill_market_order(&mut self, market_order: &mut Order) {
        match market_order.bid_or_ask {
            BidOrAsk::Ask => {
                for limit_order in self.bid_limits() {
                    limit_order.fill_order(market_order);

                    if market_order.is_filled() {
                        break;
                    }
                }
            }
            BidOrAsk::Bid => {
                for limit_order in self.ask_limits() {
                    limit_order.fill_order(market_order);
                    if market_order.is_filled() {
                        break;
                    }
                }
            }
        }
    }

    pub fn ask_limits(&mut self) -> Vec<&mut Limit> {
        self.asks.values_mut().collect()
    }
    pub fn bid_limits(&mut self) -> Vec<&mut Limit> {
        self.bids.values_mut().collect()
    }
    pub fn add_order(&mut self, price: f64, order: Order) {
        match order.bid_or_ask {
            BidOrAsk::Bid => {
                let price = Price::new(price);
                match self.bids.get_mut(&price) {
                    Some(limit) => {
                        limit.add_order(order);
                    }
                    None => {
                        let mut limit = Limit::new(price);
                        limit.add_order(order);
                        self.bids.insert(price, limit);
                    }
                }
            }
            BidOrAsk::Ask => {
                let price = Price::new(price);
                match self.asks.get_mut(&price) {
                    Some(limit) => {
                        limit.add_order(order);
                    }
                    None => {
                        let mut limit = Limit::new(price);
                        limit.add_order(order);
                        self.asks.insert(price, limit);
                    }
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Price {
    integral: u64,
    fractional: u64,
    scalar: u64,
}

impl Price {
    fn new(price: f64) -> Price {
        let scalar = 100_000;
        let integral = price as u64;
        let fractional = ((price % 1.0) * scalar as f64) as u64;
        Price {
            scalar,
            integral,
            fractional,
        }
    }
}
#[derive(Debug)]
struct Limit {
    price: Price,
    orders: Vec<Order>,
}

impl Limit {
    fn new(price: Price) -> Limit {
        Limit {
            price,
            orders: Vec::new(),
        }
    }
    fn add_order(&mut self, order: Order) {
        self.orders.push(order);
    }

    fn total_volume(&self) -> f64 {
        self.orders.iter().map(|order| order.size).sum()
    }

    fn fill_order(&mut self, market_order: &mut Order) {
        for limit_order in self.orders.iter_mut() {
            match market_order.size >= limit_order.size {
                true => {
                    market_order.size -= limit_order.size;
                    limit_order.size = 0.0
                }

                false => {
                    limit_order.size -= market_order.size;
                    market_order.size = 0.0
                }
            }

            if (market_order.is_filled()) {
                break;
            }
        }
    }
}

#[derive(Debug)]
pub struct Order {
    size: f64,
    bid_or_ask: BidOrAsk,
}

impl Order {
    pub fn new(bid_or_ask: BidOrAsk, size: f64) -> Order {
        Order { bid_or_ask, size }
    }

    pub fn is_filled(&self) -> bool {
        self.size == 0.0
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn total_volume() {
        let price = Price::new(99.99);
        let mut limit = Limit::new(price);
        assert_eq!(limit.total_volume(), 0.0);

        let limit_buy_order_1 = Order::new(BidOrAsk::Bid, 200.0);
        let limit_buy_order_2 = Order::new(BidOrAsk::Bid, 300.0);

        limit.add_order(limit_buy_order_1);
        assert_eq!(limit.total_volume(), 200.0);

        limit.add_order(limit_buy_order_2);
        assert_eq!(limit.total_volume(), 500.0);

        let mut market_sell_order = Order::new(BidOrAsk::Ask, 100.0);

        limit.fill_order(&mut market_sell_order);

        assert_eq!(limit.total_volume(), 400.0);
    }

    #[test]
    fn fill_limit_order_single() {
        let price = Price::new(99.99);
        let mut limit = Limit::new(price);
        let limit_buy_order = Order::new(BidOrAsk::Bid, 100.0);
        limit.add_order(limit_buy_order);

        let mut market_sell_order = Order::new(BidOrAsk::Ask, 99.0);

        limit.fill_order(&mut market_sell_order);

        assert_eq!(market_sell_order.is_filled(), true);
        assert_eq!(limit.orders.get(0).unwrap().size, 1.0);
    }
    #[test]
    fn fill_limit_order_multi() {
        let price = Price::new(99.99);
        let mut limit = Limit::new(price);
        let limit_buy_order_1 = Order::new(BidOrAsk::Bid, 100.0);
        let limit_buy_order_2 = Order::new(BidOrAsk::Bid, 100.0);
        limit.add_order(limit_buy_order_1);
        limit.add_order(limit_buy_order_2);

        let mut market_sell_order = Order::new(BidOrAsk::Ask, 199.0);

        limit.fill_order(&mut market_sell_order);

        assert_eq!(market_sell_order.is_filled(), true);
        assert_eq!(limit.orders.get(0).unwrap().is_filled(), true);
        assert_eq!(limit.orders.get(1).unwrap().is_filled(), false);
        assert_eq!(limit.orders.get(1).unwrap().size, 1.0);
    }
}
