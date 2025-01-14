#![allow(dead_code)]
use rust_decimal::Decimal;

use super::orderbook::OrderBook;
use super::types::order::OrderRecord;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result;

/// Represents a trading pair in a cryptocurrency or traditional market
///
/// # Fields
/// * `base` - The base currency/asset (e.g., BTC in BTC/USD)
/// * `quote` - The quote currency/asset (e.g., USD in BTC/USD)
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct TradingPair {
    base: String,
    quote: String,
}

impl TradingPair {
    pub fn new(base: String, quote: String) -> TradingPair {
        TradingPair { base, quote }
    }
}

impl Display for TradingPair {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}/{}", self.base, self.quote)
    }
}

pub struct MatchingEngine {
    orderbooks: HashMap<TradingPair, OrderBook>,
}

impl MatchingEngine {
    pub fn new() -> MatchingEngine {
        MatchingEngine {
            orderbooks: HashMap::new(),
        }
    }

    pub fn add_market(&mut self, pair: TradingPair) {
        self.orderbooks.insert(pair.clone(), OrderBook::new());
        println!("Opening a new orderbook for market {:?}", pair.to_string());
    }

    pub fn place_limit_order(
        &mut self,
        pair: TradingPair,
        price: Decimal,
        order: OrderRecord,
    ) -> Result<(), String> {
        match self.orderbooks.get_mut(&pair) {
            Some(orderbook) => {
                orderbook.add_limit_order(price, order);
                println!("Placed market order at {}", price);
                Ok(())
            }
            None => Err(format!(
                "the orderbook for the given trading pair {} is not available",
                pair.to_string()
            )),
        }
    }
}
