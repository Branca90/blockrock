use rocket::serde::{json::Json, Serialize};
use rocket::fs::FileServer;
use rocket::State;
use std::sync::Arc;
use tokio::sync::Mutex;
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;

use blockrock_core::{block::Block, transaction::Transaction, blockchain::Blockchain};
mod tron;

#[macro_use] extern crate rocket;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct BalanceResponse {
    balance: String,
}

#[get("/blocks")]
async fn get_blocks(blockchain: &State<Arc<Mutex<Blockchain>>>) -> Json<Vec<Block>> {
    let blockchain = blockchain.lock().await;
    Json(blockchain.get_blocks())
}

#[get("/balances")]
async fn get_balances(blockchain: &State<Arc<Mutex<Blockchain>>>) -> Json<Vec<(String, f64)>> {
    let blockchain = blockchain.lock().await;
    Json(blockchain.get_balances())
}

#[get("/tronbalance/<address>")]
async fn tron_balance(address: &str) -> Json<BalanceResponse> {
    // Sostituisci con la tua API key TronGrid!
    let apikey = "YOUR_TRONGRID_API_KEY";
    let balance = tron::get_tron_balance(address, apikey)
        .await
        .unwrap_or_else(|_| "Error fetching balance".to_string());
    Json(BalanceResponse { balance })
}

#[get("/")]
fn index() -> &'static str {
    "BlockRock orchestratore attivo!"
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // Inizializza chiavi e blockchain demo
    let mut rng = OsRng;
    let alice_key = SigningKey::generate(&mut rng);
    let bob_key = SigningKey::generate(&mut rng);
    let charlie_key = SigningKey::generate(&mut rng);
    let node1_key = SigningKey::generate(&mut rng);

    let mut blockchain = Blockchain::new("Node1".to_string());
    blockchain.add_public_key("Alice", alice_key.verifying_key());
    blockchain.add_public_key("Bob", bob_key.verifying_key());
    blockchain.add_public_key("Charlie", charlie_key.verifying_key());
    blockchain.add_public_key("Node1", node1_key.verifying_key());

    let tx1 = Transaction::new("Alice".to_string(), "Bob".to_string(), 30.0, &alice_key);
    blockchain.add_block(vec![tx1], "Node1".to_string());
    let tx2 = Transaction::new("Charlie".to_string(), "Alice".to_string(), 5.0, &charlie_key);
    blockchain.add_block(vec![tx2], "Node1".to_string());

    let static_path = std::env::var("STATICPATH").unwrap_or_else(|_| "static".to_string());
    let blockchain_state = Arc::new(Mutex::new(blockchain));

    rocket::build()
        .manage(blockchain_state)
        .mount("/", routes![get_blocks, get_balances, index, tron_balance])
        .mount("/static", FileServer::from(static_path))
        .configure(rocket::Config {
            address: "0.0.0.0".parse().unwrap(),
            port: 8000,
            ..Default::default()
        })
        .launch()
        .await?;

    Ok(())
}
