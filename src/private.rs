//! Gemini client for private API
use hyper::{Client, Body, Request, Uri, body::to_bytes};
use futures::Future;
use hyper::client::HttpConnector;
use std::time::Duration;
use serde::{Serialize, Deserialize};
use hyper_tls::HttpsConnector;
use crypto::{hmac::Hmac, mac::Mac, sha2::Sha384};
use hex::ToHex;
use crate::types::{GError, Response};
use crate::util::f64_from_string;
use super::structs::order::{Order, OrderSide, OrderResponse, OrderId};

pub struct Private {
    uri: String,
    api_key: String,
    api_secret: String,
    client: Client<HttpsConnector<HttpConnector>>
}

#[derive(Debug, Serialize)]
struct Payload<T: Serialize> {
    nonce: u64,
    request: String,
    #[serde(flatten)]
    content: T
}

#[derive(Debug, Serialize)]
struct PastTrades {
    symbol: String
}

#[derive(Debug, Serialize)]
struct CancelRequest {
    order_id: OrderId
}

#[derive(Debug, Deserialize)]
pub struct AccountTrade {
    #[serde(deserialize_with = "f64_from_string")]
    price: f64,

    #[serde(deserialize_with = "f64_from_string")]
    amount: f64,
    //timestamp:
    #[serde(rename="type")]
    side: OrderSide,

    #[serde(deserialize_with = "f64_from_string")]
    fee_amount: f64,
    order_id: String
}

impl Private {
    pub const USER_AGENT: &'static str =
	concat!("demo-gemini-client/", env!("CARGO_PKG_VERSION"));

    pub fn new(uri: &str, api_key: &str, api_secret: &str) -> Private {
	let https = HttpsConnector::new();
	let client = Client::builder()
	    .pool_idle_timeout(Duration::new(60, 0))
	    .build::<_, Body>(https);

	Private { uri: uri.to_string(),
		  api_key: api_key.to_string(),
		  api_secret: api_secret.to_string(),
		  client
	}
    }

    /// Create a signature based on the base64-encoded payload and the
    /// provided secret.
    fn sign(secret: &str, payload: &str) -> String {
	// hex(hmac<sha384>(payload, key=secret))
	//use hex::ToHex;
	let mut hasher = Hmac::new(Sha384::new(), secret.as_bytes());
	hasher.input(payload.as_bytes());
	let mut buf = vec![0; hasher.output_bytes()];
	hasher.raw_result(&mut buf);
	buf.encode_hex()
    }

    /// Create a request object based on the specified URI,
    /// automatically including all required headers for private
    /// requests.
    ///
    /// Lifted pretty direectly from coinbase-pro-rs.
    fn request<T: Serialize>(&self, uri: &str, body: &Payload<T>) -> anyhow::Result<Request<Body>> {
	let uri: Uri = (self.uri.to_string() + uri).parse().unwrap();

	let payload_str = serde_json::to_string(&body)?;
	let payload = base64::encode(&payload_str);
	let signature = Self::sign(&self.api_secret, &payload);

	let req = Request::post(uri)
	    .header("User-Agent", Self::USER_AGENT)
	    .header("Content-Type", "text/plain")
	    .header("X-GEMINI-APIKEY", &self.api_key)
	    .header("X-GEMINI-PAYLOAD", payload)
	    .header("X-GEMINI-SIGNATURE", signature)
	    .header("Cache-Control", "no-cache");
	Ok(req.body(Body::empty()).unwrap())
    }

    /// Lifted pretty directly from coinbase-pro-rs
    pub(crate) fn call_future<U>(
	&self,
	request: Request<Body>,
    ) -> impl Future<Output = Result<U, GError>> + 'static
    where
	for<'de> U: serde::Deserialize<'de> + 'static,
    {
	//logo::debug!("REQ: {:?}", request);

	let res = self.client.request(request);
	async move {
	    let res = res.await.map_err(GError::Http)?;
	    let body = to_bytes(res.into_body()).await.map_err(GError::Http)?;

	    let res: Result<U, GError> = serde_json::from_slice(&body).map_err(|e| {
		let err = serde_json::from_slice(&body);
		let err = err.map(GError::Gemini).unwrap_or_else(|_| {
		    let data = String::from_utf8(body.to_vec()).unwrap();
		    GError::Serde { error: e, data }
		});
		err
	    });
	    res
	}
    }

    /// Create a payload wrapping an existing (serializable) object.
    fn payload<T: Serialize>(uri: &str, x: T) -> Payload<T> {
	let nonce: i64 = chrono::Utc::now().timestamp_millis();

	Payload {
	    request: uri.to_string(),
	    nonce: nonce as u64,
	    content: x
	}
    }

    /// Return a list of recent trades.
    pub fn recent_trades(&self, symbol: &str) -> anyhow::Result<impl Response<Vec<AccountTrade>>> {
	let pt = Self::payload("/v1/mytrades", PastTrades { symbol: symbol.to_string() });
	let req = self.request(&pt.request, &pt)?;
	Ok(self.call_future(req))
    }

    /// Send a new order.
    pub fn new_order(&self, order: &Order) -> anyhow::Result<impl Response<OrderResponse>> {
	let pt = Self::payload("/v1/order/new", order);
	let req = self.request(&pt.request, &pt)?;
	Ok(self.call_future(req))
    }

    /// Cancel an order.
    pub fn cancel_order(&self, order_id: OrderId) -> anyhow::Result<impl Response<OrderResponse>> {
	let pt = Self::payload("/v1/order/cancel", CancelRequest { order_id });
	let req = self.request(&pt.request, &pt)?;
	Ok(self.call_future(req))
    }
}
