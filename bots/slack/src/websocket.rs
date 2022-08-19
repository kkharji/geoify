use std::ops::ControlFlow;

use super::{MessageEvent, PostMessageRequest, PostMessageResponse, SlackMessage};
use crate::config::Config;
use crate::protocol::{Acknowledge, Event, WSConnectResponse};
use eyre::Result;
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

    pub async fn on_message(&mut self, msg: MessageEvent<'_>) -> Result<()> {
        if msg.bot_id.is_none() {
            tracing::trace!("{msg:#?}");
            let msg_text = msg.text.to_string();
            let client = Client::builder().build()?;
            let payload = serde_json::json!({ "message": msg_text });
            let response = client
                .get("http://localhost:3291/identify")
                .json(&payload)
                .send()
                .and_then(|r| r.json::<serde_json::Value>())
                .await?;

            tracing::debug!("NLP Response {:#?}", response);
            self.post_message(msg, msg_text).await?;
        }
        Ok(())
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
                    Event::Message(msg) => {
                        if let Err(err) = self.on_message(msg).await {
                            tracing::error!("Fail to handle message: {err}")
                        }
                    }
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
    pub async fn post_message<S: AsRef<str>>(
        &mut self,
        event: MessageEvent<'_>,
        message: S,
    ) -> Result<()> {
        let request = PostMessageRequest {
            channel: event.channel,
            text: message.as_ref(),
            ..Default::default()
        };
        tracing::trace!("Sending {request:#?}");

        let response = self
            .client
            .post("https://slack.com/api/chat.postMessage")
            .json(&request)
            .send()
            .and_then(|r| r.json::<PostMessageResponse>())
            .await?;

        if !response.ok {
            eyre::bail!("post message failed: error {:?}", response.error.as_ref());
        }
        Ok(())
    }
}
