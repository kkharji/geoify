mod config;
mod logging;
mod slack;

use std::ops::ControlFlow;

use futures::{SinkExt, StreamExt};
use slack::*;
use websocket_lite::{Message, Opcode};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let config = config::Config::read();

    logging::setup(&config);

    tracing::info!("Initializing ..");

    let mut client = WSClient::new(&config).await;

    while let Some(Ok(msg)) = client.stream.next().await {
        let (data, opcode) = (msg.data(), msg.opcode());
        match opcode {
            Opcode::Text | Opcode::Binary => {
                let msg = SlackMessage::from(data);
                if let ControlFlow::Break(()) = client.handle(msg).await {
                    break;
                }
            }
            Opcode::Ping | Opcode::Close => {
                client
                    .stream
                    .send(if let Opcode::Close = opcode {
                        Message::close(None)
                    } else {
                        Message::pong(msg.into_data())
                    })
                    .await
                    .map_err(|err| eyre::eyre!("{err}"))
                    .ok();
            }
            _ => {}
        }
    }

    tracing::info!("Processing Websocket url");

    Ok(())
}
