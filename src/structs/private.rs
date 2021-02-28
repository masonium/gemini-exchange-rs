//! Structures used by the private REST client and authroized Websocket feeds.
use serde::{Serialize, Deserialize};

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
    #[serde(rename="availableForWithdrawl")]
    pub available_for_withdrawl: String,
}
