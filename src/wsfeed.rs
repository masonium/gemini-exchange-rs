//use async_trait::async_trait;
use futures::{future, Sink, Stream};
use futures_util::{sink::SinkExt, stream::TryStreamExt};
use serde_json;
use serde::Serialize;
use tokio_tungstenite::{connect_async, tungstenite::Message as TMessage};
use crate::types::GError;
use crate::structs::wsfeed::{InputMessage, Message};

pub struct WSFeed;

#[derive(Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionType {
    L2,
}

#[derive(Serialize, Debug, Clone)]
pub struct Subscription {
    pub name: SubscriptionType,
    pub symbols: Vec<String>,
}

#[derive(Serialize, Debug)]
struct Subscribe {
    #[serde(rename = "type")]
    sub_type: String,
    subscriptions: Vec<Subscription>
}

fn convert_msg(msg: TMessage) -> Message {
    match msg {
	TMessage::Text(str) => serde_json::from_str::<InputMessage>(&str)
	    .map(|x| x.into())
	    .unwrap_or_else(|e| {
	    Message::InternalError(GError::Serde {
		error: e,
		data: str,
	    })
	}),
	_ => unreachable!(), // filtered in stream
    }
}

impl<T> GeminiStream for T where T: Stream<Item = Result<Message, GError>> + Unpin + Send {}
pub trait GeminiStream: Stream<Item = Result<Message, GError>> + Unpin + Send {}
impl<T> GeminiSink for T where T: Sink<TMessage> + Unpin + Send {}
pub trait GeminiSink: Sink<TMessage> + Unpin + Send {}


impl WSFeed {
    pub async fn connect_public_data(uri: &str, subscriptions: &[Subscription]) -> anyhow::Result<impl GeminiStream + GeminiSink, GError> {
	let url = uri.to_string() + "/v2/marketdata";
	let sub = Subscribe {
	    sub_type: "subscribe".to_string(),
	    subscriptions: subscriptions.iter().cloned().collect()
	};

	let (stream, _resp) = connect_async(url)
	    .await
	    .map_err(GError::Websocket)?;

	let mut stream = stream
	    .try_filter(|msg| future::ready(msg.is_text()))
	    .map_ok(convert_msg)
	    .sink_map_err(GError::Websocket)
	    .map_err(GError::Websocket);

	let subscribe_msg = serde_json::to_string(&sub).unwrap();
	stream.send(TMessage::text(subscribe_msg)).await?;

	Ok(stream)
    }
}
