use rocket::{get, State};
use rocket::serde::json::Json; // Corretto da serde::json::Json
use blockrock_core::{block::Block, blockchain::Blockchain};
use std::sync::Arc;
use tokio::sync::Mutex;

#[get("/blocks", format = "json")]
pub async fn get_blocks(state: &State<Arc<Mutex<Blockchain>>>) -> Json<Vec<Block>> {
    let blockchain = state.lock().await;
    Json(blockchain.blocks.clone())
}

#[get("/balances", format = "json")]
pub async fn get_balances(state: &State<Arc<Mutex<Blockchain>>>) -> Json<Vec<(String, f64)>> {
    let blockchain = state.lock().await;
    let balances = blockchain.get_balances();
    Json(balances)
}

#[get("/tron_balance", format = "json")]
pub fn tron_balance() -> Json<f64> {
    Json(100.0) // Placeholder
}

#[get("/health")]
pub fn health() -> &'static str {
    "OK"
}
