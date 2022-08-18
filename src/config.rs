use color_eyre::Result;
use eyre::Context;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub slack_api_toekn: String,
    pub rust_log: String,
}

impl Config {
    pub fn from_env() -> Result<Config> {
        dotenv::dotenv().ok();

        let config = config::Config::builder()
            .add_source(config::Environment::default())
            .build()?
            .try_deserialize::<Config>()
            .wrap_err("Unable to read configuration")?;

        tracing::debug!("Load Configuration & Setup logger");

        Ok(config)
    }
}

#[test]
fn read_config_for_env() -> Result<()> {
    let config = Config::from_env();
    assert!(config.is_ok(), "{config:?}");
    Ok(())
}
