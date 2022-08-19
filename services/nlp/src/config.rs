use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub rust_log: String,
}

impl Config {
    pub fn read() -> Config {
        dotenv::from_filename("./services/nlp/.env").ok();
        let config = config::Config::builder()
            .add_source(config::Environment::default())
            .build()
            .expect("Build Configuration")
            .try_deserialize::<Config>()
            .expect("Deserialize Configuration");

        tracing::debug!("Load Configuration & Setup logger");

        config
    }
}
