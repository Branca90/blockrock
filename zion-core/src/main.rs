#[macro_use]
extern crate rocket;

use rocket::serde::{json::Json, Serialize};
use env_logger;
use log::info;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct HealthResponse {
    status: String,
}

#[get("/health")]
async fn health() -> Json<HealthResponse> {
    info!("Richiesta ricevuta per /health");
    Json(HealthResponse {
        status: "ok".to_string(),
    })
}

#[get("/")]
async fn index() -> &'static str {
    info!("Richiesta ricevuta per /");
    "Zion Core is forging towards the light!"
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // Inizializza env_logger
    env_logger::init();
    info!("Avvio del server Zion Core...");

    // Configura e avvia Rocket
    rocket::build()
        .mount("/", routes![health, index])
        .configure(
            rocket::Config::figment()
                .merge(("address", "0.0.0.0"))
                .merge(("port", 8001)),
        )
        .launch()
        .await
        .map(|_| ())
}
