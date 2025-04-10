#[macro_use] extern crate rocket;
use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use rocket::tokio::sync::Mutex;
use rocket::fs::FileServer;
use rocket::serde::{Serialize, json::Json};
use sha2::{Sha256, Digest};
use std::collections::HashMap;

#[derive(Serialize, Debug, Clone)]
struct Block {
    transactions: Vec<Transaction>,
    hash: String,
}

#[derive(Serialize, Debug, Clone)]
struct Transaction {
    from: String,
    to: String,
    amount: f64,
}

impl Transaction {
    fn new(from: String, to: String, amount: f64, _key: &SigningKey) -> Self {
        Transaction { from, to, amount }
    }
}

struct Blockchain {
    authority: String,
    public_keys: HashMap<String, VerifyingKey>,
    blocks: Vec<Vec<Transaction>>,
}

impl Blockchain {
    fn new(authority: String) -> Self {
        Blockchain {
            authority,
            public_keys: HashMap::new(),
            blocks: Vec::new(),
        }
    }

    fn add_public_key(&mut self, name: &str, key: VerifyingKey) {
        self.public_keys.insert(name.to_string(), key);
    }

    fn add_block(&mut self, transactions: Vec<Transaction>, authority: String) {
        if authority == self.authority {
            self.blocks.push(transactions);
        }
    }

    fn get_blocks(&self) -> Vec<Block> {
        self.blocks.iter().enumerate().map(|(i, txs)| {
            let mut hasher = Sha256::new();
            hasher.update(format!("{:?}{}", txs, i));
            let hash = format!("{:x}", hasher.finalize());
            Block {
                transactions: txs.to_vec(),
                hash,
            }
        }).collect()
    }

    fn get_balances(&self) -> HashMap<String, f64> {
        let mut balances: HashMap<String, f64> = HashMap::new();
        for block in &self.blocks {
            for tx in block {
                *balances.entry(tx.from.clone()).or_insert(0.0) -= tx.amount;
                *balances.entry(tx.to.clone()).or_insert(0.0) += tx.amount;
            }
        }
        balances
    }
}

#[get("/blocks")]
async fn get_blocks(blockchain: &rocket::State<Mutex<Blockchain>>) -> Json<Vec<Block>> {
    let blockchain = blockchain.lock().await;
    Json(blockchain.get_blocks())
}

#[get("/balances")]
async fn get_balances(blockchain: &rocket::State<Mutex<Blockchain>>) -> Json<HashMap<String, f64>> {
    let blockchain = blockchain.lock().await;
    Json(blockchain.get_balances())
}

#[get("/")]
fn index() -> &'static str {
    "BlockRock API"
}

#[launch]
fn rocket() -> _ {
    println!("Avvio di BlockRock con Rocket...\n");

    let authority = "Node1".to_string();
    let mut blockchain = Blockchain::new(authority.clone());

    let mut csprng = OsRng;

    let alice_key = SigningKey::generate(&mut csprng);
    let bob_key = SigningKey::generate(&mut csprng);
    let charlie_key = SigningKey::generate(&mut csprng);

    println!("Chiave pubblica di Alice: {:?}", alice_key.verifying_key());
    println!("Chiave pubblica di Bob: {:?}", bob_key.verifying_key());
    println!("Chiave pubblica di Charlie: {:?}", charlie_key.verifying_key());
    println!();

    blockchain.add_public_key("Alice", alice_key.verifying_key());
    blockchain.add_public_key("Bob", bob_key.verifying_key());
    blockchain.add_public_key("Charlie", charlie_key.verifying_key());

    let tx1 = Transaction::new("Alice".to_string(), "Bob".to_string(), 30.0, &alice_key);
    blockchain.add_block(vec![tx1], authority.clone());

    let tx2 = Transaction::new("Charlie".to_string(), "Alice".to_string(), 5.0, &charlie_key);
    blockchain.add_block(vec![tx2], authority);

    rocket::build()
        .manage(Mutex::new(blockchain))
        .mount("/", routes![get_blocks, get_balances, index])
        .mount("/static", FileServer::from("/data/local/tmp/static"))
        .configure(rocket::Config::figment()
            .merge(("address", "0.0.0.0"))
            .merge(("port", 8000)))
}
