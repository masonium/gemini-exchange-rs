//use async_trait::async_trait;
use crate::structs::wsfeed::{
    InputMDMessage, InputOrderMessage, MarketDataMessage, OrderMessage, OrderStatus,
};
use crate::types::GError;
use crate::{structs::private::Payload, Private};
use futures::{future, Sink, Stream};
use futures_util::{sink::SinkExt, stream::TryStreamExt};
use serde::Serialize;
use tokio_tungstenite::{connect_async, tungstenite::Message as TMessage};

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
    subscriptions: Vec<Subscription>,
}

fn convert_md_msg(msg: TMessage) -> MarketDataMessage {
    match msg {
        TMessage::Text(str) => serde_json::from_str::<InputMDMessage>(&str)
            .map(|x| x.into())
            .unwrap_or_else(|e| {
                MarketDataMessage::InternalError(GError::SerdeDe {
                    error: e,
                    data: str,
                })
            }),
        _ => unreachable!(), // filtered in stream
    }
}

fn convert_order_msg(msg: TMessage) -> OrderMessage {
    match msg {
        TMessage::Text(str) => {
            if let Ok(des) = serde_json::from_str::<InputOrderMessage>(&str) {
                des.into()
            } else {
                // try to deserialize a list of order status messages
                match serde_json::from_str::<Vec<OrderStatus>>(&str) {
                    Ok(orders) => OrderMessage::Orders(orders),
                    Err(e) => OrderMessage::InternalError(GError::SerdeDe {
                        error: e,
                        data: str,
                    }),
                }
            }
        }
        _ => unreachable!(), // filtered in stream
    }
}

pub trait GeminiStream<A: Sized>: Stream<Item = Result<A, GError>> + Unpin + Send {}
impl<T, A> GeminiStream<A> for T where T: Stream<Item = Result<A, GError>> + Unpin + Send {}
impl<T> GeminiSink for T where T: Sink<TMessage> + Unpin + Send {}
pub trait GeminiSink: Sink<TMessage> + Unpin + Send {}

impl WSFeed {
    pub async fn connect_public_data(
        uri: &str,
        subscriptions: &[Subscription],
    ) -> Result<impl GeminiStream<MarketDataMessage> + GeminiSink, GError> {
        let url = uri.to_string() + "/v2/marketdata";
        let sub = Subscribe {
            sub_type: "subscribe".to_string(),
            subscriptions: subscriptions.to_vec()
        };

        let (stream, _resp) = connect_async(url).await.map_err(GError::Websocket)?;

        let mut stream = stream
            .try_filter(|msg| future::ready(msg.is_text()))
            .map_ok(convert_md_msg)
            .sink_map_err(GError::Websocket)
            .map_err(GError::Websocket);

        let subscribe_msg = serde_json::to_string(&sub).unwrap();
        stream.send(TMessage::text(subscribe_msg)).await?;

        Ok(stream)
    }

    pub async fn connect_private_order_events(
        uri: &str,
        api_key: &str,
        api_secret: &str,
    ) -> Result<impl GeminiStream<OrderMessage>, GError> {
        let endpoint = "/v1/order/events";
        let url = uri.to_string() + endpoint;
        let payload = {
            let body = Payload::empty(&endpoint);
            let payload_str = serde_json::to_string(&body).map_err(GError::SerdeSer)?;
            base64::encode(&payload_str)
        };

        let signature = Private::sign(api_secret, &payload);

        let req = hyper::Request::builder()
            .uri(url)
            .header("Content-Type", "text/plain")
            .header("X-GEMINI-APIKEY", api_key)
            .header("X-GEMINI-PAYLOAD", &payload)
            .header("X-GEMINI-SIGNATURE", &signature)
            .body(())
            .unwrap();

        let (stream, _resp) = connect_async(req)
            .await
            .map_err(GError::Websocket)
            .expect("connect-async");

        let stream = stream
            .try_filter(|msg| future::ready(msg.is_text()))
            .map_ok(convert_order_msg)
            .sink_map_err(GError::Websocket)
            .map_err(GError::Websocket);

        Ok(stream)
    }
}
