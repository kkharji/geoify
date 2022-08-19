use std::ops::ControlFlow;

// use eyre::Result;
use super::{MessageEvent, PostMessageRequest, PostMessageResponse, SlackMessage};
use crate::config::Config;
use crate::protocol::{Acknowledge, Event, WSConnectResponse};
use futures::{SinkExt, TryFutureExt};
use reqwest::Client;
use tokio_util::codec::Framed;
use websocket_lite::{AsyncNetworkStream, ClientBuilder, MessageCodec};

pub type WSClientFrame = Framed<Box<dyn AsyncNetworkStream + Sync + Send + Unpin>, MessageCodec>;

pub struct WSClient {
    pub stream: WSClientFrame,
    pub client: Client,
}

impl WSClient {
    pub async fn new(config: &Config) -> Self {
        let client = Client::builder()
            .default_headers(config.client_headers())
            .build()
            .expect("Setup Slack Web Client");

        let ws_url = {
            let connection_open = client
                .post("https://slack.com/api/apps.connections.open")
                .bearer_auth(&config.slack_socket_toekn)
                .send()
                .await
                .expect("Send Post Request")
                .json::<WSConnectResponse>()
                .await
                .expect("Parse WSConnectResponse");
            if !connection_open.ok {
                let err = connection_open.error.unwrap_or_default();
                panic!("Failed to get websocket url: {err}");
            };
            connection_open.url
        };

        tracing::info!("Connecting to websocket url");

        let builder = ClientBuilder::new(&ws_url).expect("Build Client");
        let stream = builder.async_connect().await.expect("Async Connect");

        Self { stream, client }
    }

    pub async fn on_message(&mut self, msg: MessageEvent<'_>) {
        tracing::trace!("{msg:#?}");
        let msg_text = msg.text.to_string();
        tracing::debug!("Message Content {msg_text:?}");
        if msg.bot_id.is_none() {
            self.post_message(msg, msg_text).await
        }
    }

    pub async fn handle(&mut self, msg: SlackMessage<'_>) -> ControlFlow<()> {
        match msg {
            SlackMessage::Hello(msg) => {
                tracing::info!(
                    "Number of Connections: {:?}, App ID: {:?}",
                    msg.num_connections,
                    msg.connection_info.app_id,
                );
            }
            SlackMessage::EventsApi(event) => {
                let id = event.envelope_id.split("-").next().unwrap();
                tracing::info!(id, team_id = event.payload.team_id, "Envelope Received");

                let envelope_id = event.envelope_id;
                self.stream
                    .send(Acknowledge::new(envelope_id, None).as_message())
                    .await
                    .map_err(|err| tracing::error!("Failed to send payload: {err}"))
                    .ok();

                match event.payload.event {
                    Event::Message(msg) => self.on_message(msg).await,
                    Event::ReactionAdded(_) => {}
                    Event::ReactionRemoved(_) => {}
                }
            }
            SlackMessage::Disconnect(msg) => {
                tracing::info!("Received Disconnect Message: reason {}", msg.reason);
                return ControlFlow::Break(());
            }
            _ => {}
        }
        return ControlFlow::Continue(());
    }

    /// Send Reply to Slack
    pub async fn post_message<S: AsRef<str>>(&mut self, event: MessageEvent<'_>, message: S) {
        let request = PostMessageRequest {
            channel: event.channel,
            text: message.as_ref(),
            ..Default::default()
        };
        tracing::trace!("Sending {request:#?}");

        let response = match self
            .client
            .post("https://slack.com/api/chat.postMessage")
            .json(&request)
            .send()
            .and_then(|r| r.json::<PostMessageResponse>())
            .await
        {
            Ok(r) => r,
            Err(err) => {
                tracing::error!("Failed to send post message request {err}");
                return;
            }
        };

        if !response.ok {
            tracing::error!("post message failed: error {:?}", response.error.as_ref());
        }

        tracing::trace!("{response:#?}")
    }
}
