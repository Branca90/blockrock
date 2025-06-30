use rocket::{get, serde::json::Json, State};
use rocket::http::Status;
use zion_core::config::Config;
use blockrock_core::{block::Block, blockchain::Blockchain, transaction::Transaction};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(rocket::serde::Serialize)]
#[serde(crate = "rocket::serde")]
pub struct BalanceResponse {
    balance: String,
}

#[get("/blocks")]
pub async fn get_blocks(blockchain: &State<Arc<Mutex<Blockchain>>>) -> Json<Vec<Block>> {
    let blockchain = blockchain.lock().await;
    Json(blockchain.get_blocks().clone())
}

#[get("/balances")]
pub async fn get_balances(blockchain: &State<Arc<Mutex<Blockchain>>>) -> Json<Vec<(String, f64)>> {
    let blockchain = blockchain.lock().await;
    Json(blockchain.get_balances().clone())
}

#[get("/tronbalance/<address>")]
pub async fn tron_balance(address: &str, config: &State<Config>) -> Result<Json<BalanceResponse>, Status> {
    if !is_valid_tron_address(address) {
        return Err(Status::BadRequest);
    }
    let balance = super::tron::get_tron_balance(address, &config.trongrid_api_key)
        .await
        .map_err(|_| Status::InternalServerError)?;
    Ok(Json(BalanceResponse { balance }))
}

fn is_valid_tron_address(address: &str) -> bool {
    // Semplice validazione Base58
    address.len() == 34 && address.starts_with('T')
}
