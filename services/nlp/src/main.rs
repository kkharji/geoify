mod config;
mod logging;

use actix_web::web::{self, Json};
use actix_web::{App, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct NlpIdentifyRequest {
    pub message: String,
}

#[derive(Serialize)]
pub struct NlpIdentifyResponse {
    pub city: String,
    pub time: bool,
    pub weather: bool,
}

#[actix_web::get("/identify")]
async fn identify(body: Json<NlpIdentifyRequest>) -> impl Responder {
    Json(NlpIdentifyResponse {
        city: "tokyo".into(),
        time: true,
        weather: true,
    })
}

#[actix_web::main]
async fn main() {
    let config = config::Config::read();

    logging::setup(&config);

    let server = HttpServer::new(|| {
        App::new()
            .route("/ping", web::get().to(|| async { "pong" }))
            .service(identify)
    })
    .bind((config.host.as_str(), config.port))
    .expect("Bind host")
    .run();
    tracing::info!("Listening on {}:{}", config.host, config.port);

    server.await.expect("Service Run");
}
