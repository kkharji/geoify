use std::io;
use tracing::subscriber::set_global_default;
use tracing_subscriber::filter::Directive;
use tracing_subscriber::fmt::Layer;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::{registry, EnvFilter};

/// Setup tracing
pub fn setup(config: &crate::config::Config) {
    color_eyre::install().expect("Install color eyre");
    let directives: Vec<Directive> = config
        .rust_log
        .split(",")
        .map(|v| v.parse())
        .flatten()
        .collect();

    let mut filter = EnvFilter::try_from_default_env().unwrap();

    for d in directives {
        filter = filter.add_directive(d);
    }

    set_global_default(
        registry().with(filter).with(
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
