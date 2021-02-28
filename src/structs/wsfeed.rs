use serde::Deserialize;
use serde_tuple::Deserialize_tuple;
use crate::structs::OrderSide;
use crate::types::GError;
use crate::structs::order::order_side_lowercase;

#[derive(Deserialize, Debug, Clone)]
pub struct Trade {
    price: String,
    quantity: String,
    #[serde(deserialize_with = "order_side_lowercase")]
    side: OrderSide
}

#[derive(Deserialize_tuple, Debug, Clone)]
pub struct Level2Change {
    #[serde(deserialize_with = "order_side_lowercase")]
    order: OrderSide,
    price: String,
    quantity: String
}

#[derive(Deserialize, Debug)]
pub struct Candle {
    time: u64,
    open: String,
    high: String,
    low: String,
    close: String,
    volume: String
}

#[derive(Deserialize, Debug)]
pub struct AuctionEvent {

}

#[derive(Deserialize, Debug)]
pub struct Level2 {
    symbol: String,
    changes: Option<Vec<Level2Change>>,
    trades: Option<Vec<Trade>>,
    auction_events: Option<Vec<AuctionEvent>>
}

#[derive(Deserialize, Debug)]
pub struct Heartbeat {
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum InputMessage {
    L2Updates(Level2),
    Trade(Trade),
    Heartbeat(Heartbeat)
}

#[derive(Debug)]
pub enum Message {
    Level2(Level2),
    Trade(Trade),
    Heartbeat(Heartbeat),
    InternalError(GError)
}

impl From<InputMessage> for Message {
    fn from(im: InputMessage) -> Self {
	match im {
	    InputMessage::L2Updates(l2) => Message::Level2(l2),
	    InputMessage::Trade(t) => Message::Trade(t),
	    InputMessage::Heartbeat(h) => Message::Heartbeat(h)
	}
    }
}
