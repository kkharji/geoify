use serde::{Deserialize, Serialize};

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
pub enum SlackMessage<'s> {
    Hello {
        num_connections: u32,
        #[serde(borrow = "'s")]
        connection_info: ConnectionInfo<'s>,
        #[serde(borrow = "'s")]
        debug_info: DebugInfo<'s>,
    },
    Disconnect {
        reason: &'s str,
        #[serde(borrow = "'s")]
        debug_info: DebugInfo<'s>,
    },
    EventsApi {
        envelope_id: &'s str,
        #[serde(borrow = "'s")]
        payload: EventsApiPayload<'s>,
    },
}

#[derive(Serialize)]
pub struct Acknowledge<'s> {
    pub envelope_id: &'s str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<&'s str>,
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
