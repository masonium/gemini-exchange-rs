//! Structures used by the private REST client and authroized Websocket feeds.
use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct Payload<T: Serialize> {
    nonce: u64,
    pub(crate) request: String,
    #[serde(flatten)]
    content: T
}

impl<T: Serialize> Payload<T> {
    pub fn wrap(uri: &str, x: T) -> Payload<T> {
	let nonce: i64 = chrono::Utc::now().timestamp_millis();

	Payload {
	    request: uri.to_string(),
	    nonce: nonce as u64,
	    content: x
	}
    }

}
