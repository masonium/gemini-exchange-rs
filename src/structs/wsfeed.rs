use crate::structs::order::{order_id_from_string, order_side_lowercase, OrderId, OrderOption};
use crate::structs::OrderSide;
use crate::types::GError;
use serde::Deserialize;
use serde_tuple::Deserialize_tuple;

#[derive(Deserialize, Debug, Clone)]
pub struct Trade {
    price: String,
    quantity: String,
    #[serde(deserialize_with = "order_side_lowercase")]
    side: OrderSide,
}

#[derive(Deserialize_tuple, Debug, Clone)]
pub struct Level2Change {
    #[serde(deserialize_with = "order_side_lowercase")]
    pub order: OrderSide,
    pub price: String,
    pub quantity: String,
}

#[derive(Deserialize, Debug)]
pub struct Candle {
    time: u64,
    open: String,
    high: String,
    low: String,
    close: String,
    volume: String,
}

#[derive(Deserialize, Debug)]
pub struct AuctionEvent {}

#[derive(Deserialize, Debug)]
pub struct Level2 {
    pub symbol: String,
    pub changes: Option<Vec<Level2Change>>,
    pub trades: Option<Vec<Trade>>,
    pub auction_events: Option<Vec<AuctionEvent>>,
}

#[derive(Deserialize, Debug)]
pub struct Heartbeat {}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum InputMDMessage {
    L2Updates(Level2),
    Trade(Trade),
    Heartbeat(Heartbeat),
}

#[derive(Debug)]
pub enum MarketDataMessage {
    Level2(Level2),
    Trade(Trade),
    Heartbeat(Heartbeat),
    InternalError(GError),
}

impl From<InputMDMessage> for MarketDataMessage {
    fn from(im: InputMDMessage) -> Self {
        match im {
            InputMDMessage::L2Updates(l2) => MarketDataMessage::Level2(l2),
            InputMDMessage::Trade(t) => MarketDataMessage::Trade(t),
            InputMDMessage::Heartbeat(h) => MarketDataMessage::Heartbeat(h),
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Ack {
    account_id: u64,
    subscription_id: String,
}

#[derive(Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum OrderEventType {
    Initial,
    Accepted,
    Rejected,
    Booked,
    Fill,
    Cancelled,
    CancelRejected,
    Closed,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum FillLiquidity {
    Maker,
    Taker,
    Auction,
    Block,
    IndicatorOfInterest,
}

#[derive(Deserialize, Debug)]
pub struct Fill {
    pub trade_id: String,
    pub liquidity: FillLiquidity,
    pub price: String,
    pub amount: String,
    pub fee: String,
    pub fee_currency: String,
}

fn zero_price() -> String {
    "0.00".to_string()
}

#[derive(Deserialize, Debug)]
pub struct OrderStatus {
    #[serde(rename = "type")]
    pub event_type: OrderEventType,

    #[serde(deserialize_with = "order_id_from_string")]
    pub order_id: OrderId,
    pub client_order_id: Option<String>,
    pub event_id: Option<String>,
    pub api_session: Option<String>,

    pub symbol: String,

    #[serde(deserialize_with = "order_side_lowercase")]
    pub side: OrderSide,
    pub behavior: Option<OrderOption>,

    pub order_type: String,
    pub timestampms: u64,

    pub is_live: bool,
    pub is_cancelled: bool,
    pub is_hidden: bool,

    #[serde(default = "zero_price")]
    pub avg_execution_price: String,

    #[serde(default = "zero_price")]
    pub executed_amount: String,

    #[serde(default = "zero_price")]
    pub remaining_amount: String,

    pub original_amount: String,

    pub price: String,
    pub total_spend: Option<String>,

    pub reason: Option<String>,
    pub fill: Option<Fill>,
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
    Orders(Vec<OrderStatus>),
}

impl From<InputOrderMessage> for OrderMessage {
    fn from(im: InputOrderMessage) -> Self {
        match im {
            InputOrderMessage::SubscriptionAck(t) => OrderMessage::SubscriptionAck(t),
            InputOrderMessage::Heartbeat(h) => OrderMessage::Heartbeat(h),
        }
    }
}
