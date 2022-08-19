use actix_web::{web::Json, Responder};
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
pub async fn identify(body: Json<NlpIdentifyRequest>) -> impl Responder {
    Json(NlpIdentifyResponse {
        city: "tokyo".into(),
        time: true,
        weather: true,
    })
}
