use serde::Deserialize;
use serde_tuple::Deserialize_tuple;
use crate::structs::OrderSide;
use crate::types::GError;
use crate::structs::order::{order_side_lowercase, order_id_from_string, OrderId, OrderOption};

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
pub enum InputMDMessage {
    L2Updates(Level2),
    Trade(Trade),
    Heartbeat(Heartbeat)
}

#[derive(Debug)]
pub enum MarketDataMessage {
    Level2(Level2),
    Trade(Trade),
    Heartbeat(Heartbeat),
    InternalError(GError)
}

impl From<InputMDMessage> for MarketDataMessage {
    fn from(im: InputMDMessage) -> Self {
	match im {
	    InputMDMessage::L2Updates(l2) => MarketDataMessage::Level2(l2),
	    InputMDMessage::Trade(t) => MarketDataMessage::Trade(t),
	    InputMDMessage::Heartbeat(h) => MarketDataMessage::Heartbeat(h)
	}
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Ack {
    account_id: u64,
    subscription_id: String,
}

#[derive(Deserialize, Debug)]
pub struct OrderStatus {
    #[serde(rename="type")]
    pub event_type: String,

    #[serde(deserialize_with="order_id_from_string")]
    pub order_id: OrderId,
    pub client_order_id: Option<String>,
    pub event_id: Option<String>,
    pub api_session: Option<String>,

    pub symbol: String,

    #[serde(deserialize_with="order_side_lowercase")]
    pub side: OrderSide,
    pub behavior: Option<OrderOption>,

    pub order_type: String,
    pub timestampms: u64,

    pub is_live: bool,
    pub is_cancelled: bool,
    pub is_hidden: bool,

    pub avg_execution_price: String,

    pub executed_amount: String,
    pub remaining_amount: String,
    pub original_amount: String,


    pub price: String,
    pub total_spend: Option<String>,

    pub reason: Option<String>
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum InputOrderMessage {
    Heartbeat(Heartbeat),
    SubscriptionAck(Ack),
}

#[derive(Debug)]
pub enum OrderMessage {
    Heartbeat(Heartbeat),
    SubscriptionAck(Ack),
    InternalError(GError),
    Orders(Vec<OrderStatus>)
}

impl From<InputOrderMessage> for OrderMessage {
    fn from(im: InputOrderMessage) -> Self {
	match im {
	    InputOrderMessage::SubscriptionAck(t) => OrderMessage::SubscriptionAck(t),
	    InputOrderMessage::Heartbeat(h) => OrderMessage::Heartbeat(h)
	}
    }
}
