use std::ops::ControlFlow;

// use eyre::Result;
use super::{connection_open, MessageEvent, SlackMessage};
use crate::config::Config;
use crate::slack::{Acknowledge, Event};
use futures::SinkExt;
use tokio_util::codec::Framed;
use websocket_lite::{AsyncNetworkStream, ClientBuilder, MessageCodec};

pub type WSClientFrame = Framed<Box<dyn AsyncNetworkStream + Sync + Send + Unpin>, MessageCodec>;

pub struct WSClient {
    pub stream: WSClientFrame,
}

impl WSClient {
    pub async fn new(config: &Config) -> Self {
        let url = connection_open(&config).await.expect("Request WS URL");
        tracing::info!("Connecting to websocket url");

        let builder = ClientBuilder::new(&url).expect("Build Client");
        let stream = builder.async_connect().await.expect("Async Connect");

        Self { stream }
    }

    pub async fn on_message(&mut self, envelope_id: &str, msg: MessageEvent<'_>) {
        tracing::trace!("{msg:#?}");
        tracing::debug!("Message Content {:?}", msg.text);

        self.reply(envelope_id, Some("Hello")).await
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
                self.reply(event.envelope_id, None).await;
                match event.payload.event {
                    Event::Message(msg) => self.on_message(event.envelope_id, msg).await,
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
    pub async fn reply<S: AsRef<str>>(&mut self, envelope_id: S, payload: Option<S>) {
        self.stream
            .send(Acknowledge::new(envelope_id, payload).as_message())
            .await
            .map_err(|err| tracing::error!("Failed to send payload: {err}"))
            .ok();
    }
}
