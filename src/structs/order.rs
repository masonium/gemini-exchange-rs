use serde::{Deserialize, Serialize, Serializer, Deserializer, de::{self, Visitor}};
use std::fmt;

/// order id
#[derive(Deserialize, Serialize, Clone, Copy, Debug)]
pub struct OrderId(u64);

impl fmt::Display for OrderId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
	write!(f, "{}", self.0)
    }
}

struct OrderIdInQuotes;
impl<'de> Visitor<'de> for OrderIdInQuotes {
    type Value = OrderId;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
	f.write_str("i64 as a number or string")
    }

    fn visit_u64<E>(self, id: u64) -> Result<Self::Value, E>
    where
	E: de::Error,
    {
	Ok(OrderId(id))
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
	E: de::Error,
    {
	s.parse::<u64>().map(OrderId).map_err(de::Error::custom)
    }

}

/// Parse an `OrderId` from a string or u64 representation.
pub fn order_id_from_string<'de, D>(d: D) -> Result<OrderId, D::Error>
where
    D: Deserializer<'de>,
{
    d.deserialize_any(OrderIdInQuotes)
}

/// Side of the order (buy or sell)
#[derive(Serialize, Deserialize, Debug)]
pub enum OrderSide {
    Buy,
    Sell
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum OrderOptions {
    MakerOrCancel,
    ImmediateOrCancel,
    FillOrKill,
    AuctionOnly,
    IndicationOfInterest
}

impl OrderSide {
    fn lowercase<S: Serializer>(os: &OrderSide, s: S) -> Result<S::Ok, S::Error> {
	s.serialize_str(match os {
	    OrderSide::Buy => "buy",
	    OrderSide::Sell => "sell",
	})
    }
}

struct LowercaseOrderSide;

impl<'de> Visitor<'de> for LowercaseOrderSide {
    type Value = OrderSide;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
	f.write_str("'buy' or 'sell' as lowercase")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
	E: de::Error,
    {
	match s {
	    "buy" => Ok(OrderSide::Buy),
	    "sell" => Ok(OrderSide::Sell),
	    _ => Err(de::Error::custom(s))
	}
    }
}

pub fn order_side_lowercase<'de, D>(d: D) -> Result<OrderSide, D::Error>
where
    D: Deserializer<'de>,
{
    d.deserialize_any(LowercaseOrderSide)
}

#[derive(Debug)]
pub enum OrderType {
    Limit,
    StopLimit
}

impl Serialize for OrderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
	S: Serializer,
    {
	serializer.serialize_str(match self {
	    OrderType::Limit => "exchange limit",
	    OrderType::StopLimit => "exchange stop limit",
	})
    }
}

#[derive(Serialize, Debug)]
pub struct Order {
    price: String,
    amount: String,

    #[serde(serialize_with="OrderSide::lowercase")]
    side: OrderSide,
    symbol: String,
    client_order_id: String,
    options: Vec<OrderOptions>,

    #[serde(rename="type")]
    order_type: OrderType,
}

impl Order {
    fn format_prices(symbol: &str, size: f64, price: f64) -> (String, String) {
	match symbol {
	    "btcusd" => (format!("{:.8}", size), format!("{:.2}", price)),
	    "ltcusd" => (format!("{:.5}", size), format!("{:.2}", price)),
	    "ethusd" => (format!("{:.6}", size), format!("{:.2}", price)),
	    _ => panic!("unknown symbol for formatting: {}", symbol)
	}
    }

    pub fn limit(symbol: &str, client_oid: String,
		 side: OrderSide, size: f64, price: f64, post_only: bool) -> Order {
	let (size_str, price_str) = Self::format_prices(symbol, size, price);
	let mut options = Vec::new();
	if post_only {
	    options.push(OrderOptions::MakerOrCancel);
	}
	Order {
	    price: price_str,
	    amount: size_str,
	    side,
	    symbol: symbol.to_string(),
	    client_order_id: client_oid,
	    options,
	    order_type: OrderType::Limit
	}
    }
}

/// Response from creating or cancelling an order.
#[derive(Deserialize, Debug)]
pub struct OrderResponse {
    #[serde(deserialize_with="order_id_from_string")]
    pub order_id: OrderId,
    pub client_order_id: Option<String>,
    pub symbol: String,
    pub exchange: String,
    pub price: String,
    pub avg_execution_price: String,

    #[serde(deserialize_with="order_side_lowercase")]
    pub side: OrderSide,

    #[serde(rename="type")]
    pub order_type: String,

    pub options: Vec<OrderOptions>,

    pub is_live: bool,
    pub is_cancelled: bool,

    pub reason: Option<String>,

    pub executed_amount: String,
    pub remaining_amount: String,
    pub original_amount: String,

    pub is_hidden: bool
}
