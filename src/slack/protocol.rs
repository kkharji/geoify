use bytes::Bytes;
use serde::{Deserialize, Serialize};
use websocket_lite::Message;

#[derive(Debug, Deserialize)]
pub struct WSConnectResponse {
    /// Response status
    pub ok: bool,
    /// WebSocket Url
    pub url: String,
    /// Error Message in case of errors
    pub error: Option<String>,
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

#[derive(Serialize)]
pub struct Acknowledge {
    pub envelope_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
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

#[derive(Deserialize, Debug)]
pub struct EventsApiPayload<'s> {
    pub team_id: &'s str,
    #[serde(borrow = "'s")]
    pub event: Event<'s>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Event<'s> {
    #[serde(borrow = "'s")]
    Message(MessageEvent<'s>),
    #[serde(borrow = "'s")]
    ReactionAdded(ReactionEvent<'s>),
    #[serde(borrow = "'s")]
    ReactionRemoved(ReactionEvent<'s>),
}

#[derive(Deserialize, Debug)]
pub struct MessageEvent<'s> {
    pub event_ts: &'s str,
    pub subtype: Option<&'s str>,
    pub text: Option<std::borrow::Cow<'s, str>>,
    pub user: Option<&'s str>,
    pub ts: Option<&'s str>,
    pub deleted_ts: Option<&'s str>,
    pub team: Option<&'s str>,
    pub channel: &'s str,
    #[serde(default)]
    pub hidden: bool,
    #[serde(default)]
    pub is_starred: bool,
    #[serde(default)]
    pub pinned_to: Vec<&'s str>,
    #[serde(default, borrow = "'s")]
    pub reactions: Vec<MessageReaction<'s>>,
}

#[derive(Deserialize, Debug)]
pub struct MessageReaction<'s> {
    pub name: &'s str,
    pub count: u32,
    #[serde(default, borrow = "'s")]
    pub users: Vec<&'s str>,
}

#[derive(Deserialize, Debug)]
pub struct ReactionEvent<'s> {
    pub event_ts: &'s str,
    pub user: &'s str,
    pub reaction: &'s str,
    pub item_user: Option<&'s str>,
    #[serde(borrow = "'s")]
    pub item: ReactionItem<'s>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum ReactionItem<'s> {
    Message {
        channel: &'s str,
        ts: &'s str,
    },
    File {
        file: &'s str,
    },
    FileComment {
        file_comment: &'s str,
        file: &'s str,
    },
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
