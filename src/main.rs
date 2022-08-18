mod config;
mod logging;
mod slack;

use futures::sink::SinkExt;
use futures::stream::StreamExt;
use slack::*;
use websocket_lite::{ClientBuilder, Message, Opcode};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let config = config::Config::read();

    logging::setup(&config);
    tracing::info!("Initializing ..");

    let url = connection_open(&config).await?;

    tracing::info!("Processing Websocket url");

    let builder = ClientBuilder::new(&url).expect("Build Client");
    let mut stream_mut = builder.async_connect().await.expect("Async Connect");

    while let (Some(msg), mut stream) = stream_mut.into_future().await {
        match msg {
            Ok(msg) => {
                let (data, opcode) = (msg.data(), msg.opcode());

                match opcode {
                    Opcode::Text | Opcode::Binary => {
                        match serde_json::from_slice::<SlackMessage>(data) {
                            Ok(m) => match m {
                                SlackMessage::Hello {
                                    num_connections,
                                    connection_info: ConnectionInfo { app_id },
                                    ..
                                } => {
                                    tracing::info!("Received Hello Message");
                                    tracing::info!(
                                        "Number of Connections: {num_connections:?}, App ID: {app_id:#?}"
                                    );
                                }
                                SlackMessage::EventsApi {
                                    envelope_id,
                                    payload,
                                } => {
                                    let id = envelope_id.split("-").next().unwrap();
                                    tracing::info!(id, "Envelope Received");
                                    let EventsApiPayload { team_id, event } = payload;
                                    tracing::info!(id, team_id, "{event:#?}");
                                    // span.in_scope(|| {
                                    // })
                                }
                                SlackMessage::Disconnect { .. } => {
                                    tracing::info!("Received Disconnect Message");
                                }
                            },
                            Err(err) => {
                                tracing::error!(
                                    "Failed to parse incoming message: {data:?}: {err:?}"
                                )
                            }
                        };
                    }
                    Opcode::Ping => stream
                        .send(Message::pong(msg.into_data()))
                        .await
                        .map_err(|err| eyre::eyre!("{err}"))?,
                    Opcode::Close => stream
                        .send(Message::close(None))
                        .await
                        .map_err(|err| eyre::eyre!("{err}"))?,
                    Opcode::Pong => (),
                }

                stream_mut = stream;
            }

            Err(err) => {
                tracing::error!("Stream returned unexpected {err}!");
                let _ = stream.send(Message::close(None)).await;
                break;
            }
        }
    }

    Ok(())
}
