#[macro_use]
extern crate rocket;

use rocket::State;
use rocket::serde::json::Json;
use rocket::fs::FileServer;
use rocket::response::Redirect;
use std::sync::Mutex;
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;
mod transaction;
mod block;
mod blockchain;
use transaction::Transaction;
use blockchain::Blockchain;
use std::collections::HashMap;

#[get("/blocks")]
fn get_blocks(blockchain: &State<Mutex<Blockchain>>) -> Json<Vec<block::Block>> {
    let blockchain = blockchain.lock().unwrap();
    Json(blockchain.blocks.clone())
}

#[get("/balances")]
fn get_balances(blockchain: &State<Mutex<Blockchain>>) -> Json<HashMap<String, f64>> {
    let blockchain = blockchain.lock().unwrap();
    Json(blockchain.balances.clone())
}

#[get("/")]  // Nuova rotta per la root
fn index() -> Redirect {
    Redirect::to("/static/index.html")
}

#[launch]
fn rocket() -> _ {
    println!("Avvio di BlockRock con Rocket...\n");

    let authority = "Node1".to_string();
    let mut blockchain = Blockchain::new(authority.clone());

    let mut csprng = OsRng;

    // Generazione chiavi
    let alice_key = SigningKey::generate(&mut csprng);
    let bob_key = SigningKey::generate(&mut csprng);
    let charlie_key = SigningKey::generate(&mut csprng);

    // Registrazione chiavi pubbliche
    println!("Chiave pubblica di Alice: {:?}", alice_key.verifying_key());
    println!("Chiave pubblica di Bob: {:?}", bob_key.verifying_key());
    println!("Chiave pubblica di Charlie: {:?}", charlie_key.verifying_key());
    println!();

    blockchain.add_public_key("Alice", alice_key.verifying_key());
    blockchain.add_public_key("Bob", bob_key.verifying_key());
    blockchain.add_public_key("Charlie", charlie_key.verifying_key());

    // Aggiungi alcune transazioni di esempio
    let tx1 = Transaction::new("Alice".to_string(), "Bob".to_string(), 30.0, &alice_key);
    blockchain.add_block(vec![tx1], authority.clone());

    let tx2 = Transaction::new("Charlie".to_string(), "Alice".to_string(), 5.0, &charlie_key);
    blockchain.add_block(vec![tx2], authority);

    let static_path = env!("CARGO_MANIFEST_DIR").to_string() + "/static";
    rocket::build()
        .manage(Mutex::new(blockchain))
        .mount("/", routes![get_blocks, get_balances, index])  // Aggiungi "index" alle rotte
        .mount("/static", FileServer::from(static_path))
        .configure(rocket::Config::figment()
            .merge(("address", "0.0.0.0"))
            .merge(("port", 8000)))
}
