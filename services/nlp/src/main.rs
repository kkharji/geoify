mod config;
mod endpoint;
mod logging;

use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() {
    let config = config::Config::read();

    logging::setup(&config);

    let server = HttpServer::new(|| {
        App::new()
            .route("/ping", web::get().to(|| async { "pong" }))
            .service(endpoint::identify)
    })
    .bind((config.host.as_str(), config.port))
    .expect("Bind host")
    .run();
    tracing::info!("Listening on {}:{}", config.host, config.port);

    server.await.expect("Service Run");
}
