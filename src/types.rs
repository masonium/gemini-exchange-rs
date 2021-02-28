use thiserror::Error;
use futures::Future;
use serde::Deserialize;

#[derive(Error, Debug)]
pub enum GError {
    #[error("http: {0}")]
    Http(#[source] hyper::Error),

    #[error("gemini error: {0:?}")]
    Gemini(GeminiResponseError),

    #[error("deseriaization error: {error}\n{data}")]
    SerdeDe {
	#[source]
	error: serde_json::Error,

	data: String
    },

    #[error("Serialization error: {0}")]
    SerdeSer(#[source] serde_json::Error),

    #[error("websocket error: {0}")]
    Websocket(#[source] tokio_tungstenite::tungstenite::Error)
}

#[derive(Debug, Deserialize)]
pub struct GeminiResponseError {
    result: String,
    reason: String,
    message: String
}

pub type Result<T> = core::result::Result<T, GError>;

/// Future repsonse from a client.
pub trait Response<A: Sized>: Future<Output = Result<A>> {
    type Result: Sized;
}

impl<T, A: Sized> Response<A> for T where T: Future<Output = Result<A>> {
    type Result = A;
}
