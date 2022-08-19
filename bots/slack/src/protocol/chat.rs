use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize)]
pub struct PostMessageRequest<'a> {
    pub channel: &'a str,
    pub text: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_names: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unfurl_links: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unfurl_media: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mrkdwn: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_user: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_emoji: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_broadcast: Option<bool>,
}

// blocks?: (KnownBlock | Block)[];
// metadata?: MessageMetadata;
// parse?: 'full' | 'none';
// thread_ts?: string;
// unfurl_links?: boolean;

#[derive(Clone, Debug, Deserialize)]
pub struct PostMessageResponse {
    pub channel: Option<String>,
    pub error: Option<String>,
    pub message: Option<serde_json::Value>,
    #[serde(default)]
    pub ok: bool,
}
