use std::io;
use std::path::Path;
use tracing::dispatcher::SetGlobalDefaultError;
use tracing::subscriber::set_global_default;
use tracing_appender::rolling;
use tracing_subscriber::fmt::Layer;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::{registry, EnvFilter};

/// Setup tracing
pub fn setup(
    path: impl AsRef<Path>,
    with_stdout: bool,
    rust_log: &str,
) -> Result<(), SetGlobalDefaultError> {
    let path = path.as_ref();
    let root = path.parent().unwrap();
    let filename = path.file_name().unwrap().to_str().unwrap();
    let default_filter = EnvFilter::new(rust_log);

    let fmt_file = Layer::new()
        .with_writer(rolling::never(root, filename))
        .with_target(false)
        .with_file(false)
        .without_time()
        .with_thread_names(false)
        .with_thread_ids(false);
    // .with_ansi(false)
    // .compact();
    let fmt_stdout = Layer::new()
        .with_writer(io::stdout)
        .with_target(false)
        .with_line_number(true)
        .without_time()
        .with_file(true);
    // .compact();

    if with_stdout {
        set_global_default(
            registry()
                .with(default_filter)
                .with(fmt_file)
                .with(fmt_stdout),
        )?
    } else {
        set_global_default(registry().with(default_filter).with(fmt_file))?
    }
    Ok(())
}
