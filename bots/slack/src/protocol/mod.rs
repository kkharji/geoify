mod chat;
mod events;

use bytes::Bytes;
use serde::{Deserialize, Serialize};
use websocket_lite::Message;

pub use chat::*;
pub use events::*;

#[derive(Debug, Deserialize)]
pub struct WSConnectResponse {
    #[serde(default)]
    pub url: String,
    pub error: Option<String>,
    pub ok: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum SlackMessage<'s> {
    #[serde(borrow = "'s")]
    Hello(HelloMessage<'s>),
    #[serde(borrow = "'s")]
    Disconnect(DisconnectMessage<'s>),
    #[serde(borrow = "'s")]
    EventsApi(EventsApiMessage<'s>),
    None,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case", tag = "type")]
pub struct HelloMessage<'s> {
    pub num_connections: u32,
    #[serde(borrow = "'s")]
    pub connection_info: ConnectionInfo<'s>,
    #[serde(borrow = "'s")]
    pub debug_info: DebugInfo<'s>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case", tag = "type")]
pub struct DisconnectMessage<'s> {
    pub reason: &'s str,
    #[serde(borrow = "'s")]
    pub debug_info: DebugInfo<'s>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case", tag = "type")]
pub struct EventsApiMessage<'s> {
    pub envelope_id: &'s str,
    #[serde(borrow = "'s")]
    pub payload: EventsApiPayload<'s>,
}

impl<'a> From<&'a Bytes> for SlackMessage<'a> {
    fn from(value: &'a Bytes) -> Self {
        match serde_json::from_slice::<SlackMessage>(value) {
            Ok(m) => m,
            Err(err) => {
                tracing::error!("Failed to parse a message: {value:?}: {err:?}");
                SlackMessage::None
            }
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct ConnectionInfo<'s> {
    pub app_id: &'s str,
}

#[derive(Deserialize, Debug)]
pub struct DebugInfo<'s> {
    pub host: &'s str,
    pub started: Option<&'s str>,
    pub build_number: Option<u32>,
    pub approximate_connection_time: Option<u64>,
}

#[derive(Serialize)]
pub struct Acknowledge {
    pub envelope_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
}

impl Acknowledge {
    pub fn new<S: AsRef<str>>(envelope_id: S, payload: Option<S>) -> Self {
        Self {
            envelope_id: envelope_id.as_ref().into(),
            payload: payload.map(|s| s.as_ref().into()),
        }
    }
    pub fn as_message(&self) -> Message {
        let text = serde_json::to_string(self)
            .map_err(|err| {
                tracing::error!("Failed to convert Acknowledge to string {err}");
            })
            .unwrap_or_default();
        Message::text(text)
    }
}
