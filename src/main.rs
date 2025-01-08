mod matching_engine;
use matching_engine::engine::{MatchingEngine, TradingPair};
use matching_engine::orderbook::{BidOrAsk, Order, OrderBook};
fn main() {
    // let buy_order_from_naveen = Order::new(BidOrAsk::Ask, 16.0);

    // let mut orderbook = OrderBook::new();

    // orderbook.add_order(199.012, buy_order_from_naveen);
    // orderbook.add_order(199.012, buy_order_from_nik);
    // // println!("{:?}", orderbook);

    let mut engine = MatchingEngine::new();
    let base = String::from("BTC");
    let quote = String::from("USDT");
    let pair = TradingPair::new(base, quote);
    let price = 100.00;
    let buy_order_from_nik = Order::new(BidOrAsk::Ask, 141.0);
    engine.add_market(pair.clone());

    engine
        .place_limit_order(pair, price, buy_order_from_nik)
        .unwrap()
}
