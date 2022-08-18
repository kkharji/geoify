mod config;
mod logging;

fn main() -> color_eyre::Result<()> {
    let config = config::Config::try_read()?;

    logging::setup(&config.log_file, true, &config.rust_log)?;
    tracing::info!("Geoify Started at {}:{}", config.host, config.port);

    Ok(())
}
