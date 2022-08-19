use reqwest::header::HeaderMap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub slack_api_token: String,
    pub slack_socket_toekn: String,
    pub rust_log: String,
    pub log_file: String,
}

impl Config {
    pub fn read() -> Config {
        dotenv::dotenv().ok();

        let config = config::Config::builder()
            .add_source(config::Environment::default())
            .build()
            .expect("Build Configuration")
            .try_deserialize::<Config>()
            .expect("Deserialize Configuration");

        tracing::debug!("Load Configuration & Setup logger");

        config
    }

    pub fn client_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", self.slack_api_token)
                .parse()
                .expect("Parse slack api token"),
        );
        headers
    }
}

#[test]
fn read_config_for_env() {
    Config::read();
}
