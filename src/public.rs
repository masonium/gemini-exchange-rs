//! Gemini client for apps
use hyper::{Client, Body, Request, Uri, body::to_bytes};
use futures::Future;
use hyper::client::HttpConnector;
use std::time::Duration;
use hyper_tls::HttpsConnector;
use serde::Deserialize;
use std::collections::HashMap;
use crate::util::f64_from_string;
use crate::types::{GError, Result, Response};

pub struct Public {
    uri: String,
    client: Client<HttpsConnector<HttpConnector>>
}

#[derive(Deserialize, Debug)]
pub struct VolumeInfo {
    vals: HashMap<String, f64>,
}


/// Response from a Ticker request with recent information about trading.
#[derive(Deserialize, Debug)]
pub struct Ticker {
    #[serde(deserialize_with = "f64_from_string")]
    ask: f64,
    #[serde(deserialize_with = "f64_from_string")]
    bid: f64,
    #[serde(deserialize_with = "f64_from_string")]
    last: f64,
    //volume: VolumeInfo
}

impl Public {
    pub const USER_AGENT: &'static str =
	concat!("demo-gemini-client/", env!("CARGO_PKG_VERSION"));

    pub fn new(uri: &str) -> Self {
	let https = HttpsConnector::new();
	let client = Client::builder()
	    .pool_idle_timeout(Duration::new(60, 0))
	    .build::<_, Body>(https);
	let uri = uri.to_string();
	Self { uri, client }
    }


    /// Create a request object based on the specified Uri.
    ///
    /// Lifted pretty direectly from coinbase-pro-rs.
    fn request(&self, uri: &str) -> Request<Body> {
	let uri: Uri = (self.uri.to_string() + uri).parse().unwrap();

	let req = Request::get(uri).header("User-Agent", Self::USER_AGENT);
	req.body(Body::empty()).unwrap()
    }

    /// Lifted pretty directly from coinbase-pro-rs
    pub(crate) fn call_future<U>(
	&self,
	request: Request<Body>,
    ) -> impl Future<Output = Result<U>> + 'static
    where
	for<'de> U: serde::Deserialize<'de> + 'static,
    {
	//logo::debug!("REQ: {:?}", request);

	let res = self.client.request(request);
	async move {
	    let res = res.await.map_err(GError::Http)?;
	    let body = to_bytes(res.into_body()).await.map_err(GError::Http)?;
	    //log::debug!("RES: {:#?}", body);
	    let res: Result<U> = serde_json::from_slice(&body).map_err(|e| {
		let err = serde_json::from_slice(&body);
		let err = err.map(GError::Gemini).unwrap_or_else(|_| {
		    let data = String::from_utf8(body.to_vec()).unwrap();
		    GError::SerdeDe { error: e, data }
		});
		err
	    });
	    res
	}
    }

    /// Get ticker information for a product such as BTCUSD
    pub fn get_symbols(&self) -> impl Response<Vec<String>> {
	let req = self.request("/v1/symbols");
	self.call_future(req)
    }

    /// Get ticker information for a product such as BTCUSD
    pub fn get_ticker(&self, product: &str) -> impl Response<Ticker> {
	let req = self.request(&format!("/v1/pubticker/{}", product));
	self.call_future(req)
    }
}
