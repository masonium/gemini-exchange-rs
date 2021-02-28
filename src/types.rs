use thiserror::Error;
use futures::Future;

#[derive(Error, Debug)]
pub enum GError {
    #[error("http: {0}")]
    Http(#[source] hyper::Error),

    #[error("gemini error: {0}")]
    Gemini(String),

    #[error("deseriaization error: {error}\n{data}")]
    Serde {
	#[source]
	error: serde_json::Error,

	data: String
    },

    #[error("websocket error: {0}")]
    Websocket(#[source] tokio_tungstenite::tungstenite::Error)
}

/// Future repsonse from a client.
pub trait Response<A: Sized>: Future<Output = Result<A, GError>> {
    type Result: Sized;
}

impl<T, A: Sized> Response<A> for T where T: Future<Output = Result<A, GError>> {
    type Result = A;
}
