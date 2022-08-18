mod protocol;
mod websocket;
use crate::config::Config;

use eyre::Result;
pub use protocol::*;
pub use websocket::*;

/// Requests [apps.connections.open](https://api.slack.com/methods/apps.connections.open).
/// Used to get websocket url
pub async fn connection_open(config: &Config) -> Result<String> {
    let mut response = surf::post("https://slack.com/api/apps.connections.open")
        .header(
            "Authorization",
            format!("Bearer {}", config.slack_api_token),
        )
        .recv_json::<WSConnectResponse>()
        .await
        .unwrap();

    if !response.ok {
        eyre::bail!(
            "Failed to get websocket url! {}",
            response.error.unwrap_or_default()
        );
    }
    Ok(response.url)
}
