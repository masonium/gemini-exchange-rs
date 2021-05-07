//! Structures used by the private REST client and authroized Websocket feeds.
use serde::{Serialize, Deserialize};
use crate::structs::order::{OrderId, OrderSide};

/// Payload directly deliverable to the Gemini API, including common
/// wrapper fields.
#[derive(Debug, Serialize)]
pub(crate) struct Payload<T: Serialize> {
    nonce: u64,
    pub(crate) request: String,
    #[serde(flatten)]
    content: T
}

impl<T: Serialize> Payload<T> {
    /// Return a payload wrapping a deserializable structure.
    pub fn wrap(uri: &str, x: T) -> Payload<T> {
	let nonce: i64 = chrono::Utc::now().timestamp_millis();

	Payload {
	    request: uri.to_string(),
	    nonce: nonce as u64,
	    content: x
	}
    }
}

impl Payload<()> {
    /// Return a payload with no internal contents.
    pub fn empty(uri: &str) -> Payload<()> {
	Self::wrap(uri, ())
    }
}


/// Represent the current balance for a particular symbol
#[derive(Debug, Deserialize)]
pub struct AccountBalance {
    pub currency: String,
    pub amount: String,
    pub available: String,
    #[serde(rename="availableForWithdrawal")]
    pub available_for_withdrawal: String,
}


#[derive(Debug, Deserialize)]
pub struct NotionalVolume {
    date: String,
    last_updated_ms: u64,
    web_maker_fee_bps: u32,
    web_taker_fee_bps: u32,
    web_auction_fee_bps: u32,
    api_maker_fee_bps: u32,
    api_taker_fee_bps: u32,
    api_auction_fee_bps: u32,
    fix_maker_fee_bps: u32,
    fix_taker_fee_bps: u32,
    fix_auction_fee_bps: u32,
    block_maker_fee_bps: u32,
    block_taker_fee_bps: u32,
    notional_30d_volume: u32,
}

#[derive(Debug, Serialize)]
pub(crate) struct PastTrades {
    pub(crate) symbol: String
}

#[derive(Debug, Serialize)]
pub(crate) struct CancelRequest {
    pub(crate) order_id: OrderId
}

#[derive(Debug, Deserialize)]
pub struct AccountTrade {
    price: String,

    amount: String,
    //timestamp:
    #[serde(rename="type")]
    side: OrderSide,

    fee_amount: String,
    order_id: String
}

#[derive(Debug, Deserialize)]
pub struct CancelResponse {
    result: String,
    details: CancelDetails
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct CancelDetails {
    cancel_rejects: Vec<OrderId>,
    cancelled_orders: Vec<OrderId>
}
