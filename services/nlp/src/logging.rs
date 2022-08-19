use std::io;
use tracing::metadata::LevelFilter;
use tracing::subscriber::set_global_default;
use tracing_subscriber::fmt::Layer;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::{registry, EnvFilter};

pub fn setup(config: &crate::config::Config) {
    color_eyre::install().expect("Install color eyre");
    set_global_default(
        registry()
            .with(
                EnvFilter::builder()
                    .with_default_directive(LevelFilter::DEBUG.into())
                    .parse(&config.rust_log)
                    .expect("Build logs filter"),
            )
            .with(
                Layer::new()
                    .with_writer(io::stdout)
                    .with_target(false)
                    .with_line_number(true)
                    .without_time()
                    .with_file(true),
            ),
    )
    .expect("Initialize tracer")
}
