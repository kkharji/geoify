mod protocol;
mod websocket;
use crate::config::Config;
use reqwest::Client;

use eyre::{Context, Result};
pub use protocol::*;
pub use websocket::*;

/// Call [apps.connections.open](https://api.slack.com/methods/apps.connections.open).
pub async fn connection_open(config: &Config) -> Result<String> {
    Client::builder()
        .build()?
        .post("https://slack.com/api/apps.connections.open")
        .bearer_auth(&config.slack_api_token)
        .send()
        .await?
        .json::<WSConnectResponse>()
        .await
        .context("Parse WSConnectResponse")
        .and_then(|res| {
            if !res.ok {
                let err = res.error.unwrap_or_default();
                eyre::bail!("Failed to get websocket url: {err}",);
            }
            Ok(res.url)
        })
}
