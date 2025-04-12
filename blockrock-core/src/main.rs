use rocket::serde::json::Json;
use rocket::fs::FileServer;
use rocket::State;
use std::sync::Arc;
use tokio::sync::Mutex;
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;
use serde_json;

mod blockchain;
mod transaction;
mod block;
use blockchain::{Blockchain, Block, Transaction};

#[macro_use]
extern crate rocket;

#[get("/blocks")]
async fn get_blocks(blockchain: &State<Arc<Mutex<Blockchain>>>) -> Json<Vec<Block>> {
    let blockchain = blockchain.lock().await;
    let blocks = blockchain.get_blocks().clone();
    println!("Returning blocks from /blocks endpoint: {:?}", blocks);
    Json(blocks)
}

#[get("/balances")]
async fn get_balances(blockchain: &State<Arc<Mutex<Blockchain>>>) -> Json<Vec<(String, f64)>> {
    let blockchain = blockchain.lock().await;
    Json(blockchain.get_balances())
}

#[get("/")]
async fn index() -> &'static str {
    "BlockRock is running!"
}

async fn run_p2p_node(blockchain: Arc<Mutex<Blockchain>>, node1_key: SigningKey) {
    use libp2p::{
        core::upgrade,
        gossipsub, identity,
        noise, swarm::SwarmEvent,
        tcp, yamux, Swarm,
        Multiaddr, Transport,
    };
    use futures::StreamExt;

    let local_key = identity::Keypair::ed25519_from_bytes(node1_key.to_bytes())
        .expect("Invalid key bytes");
    let local_peer_id = identity::PeerId::from(local_key.public());
    println!("Local peer ID: {:?}", local_peer_id);

    let mut swarm = {
        let gossipsub_config = gossipsub::ConfigBuilder::default().build().unwrap();
        let gossipsub = gossipsub::Behaviour::<gossipsub::IdentityTransform>::new(
            gossipsub::MessageAuthenticity::Signed(local_key.clone()),
            gossipsub_config,
        )
        .unwrap();

        let transport = tcp::tokio::Transport::new(tcp::Config::default())
            .upgrade(upgrade::Version::V1)
            .authenticate(noise::Config::new(&local_key).unwrap())
            .multiplex(yamux::Config::default())
            .boxed();

        let behaviour = gossipsub;
        Swarm::new(
            transport,
            behaviour,
            local_peer_id,
            libp2p::swarm::Config::with_tokio_executor(),
        )
    };

    let topic = gossipsub::IdentTopic::new("blockrock-blocks");
    swarm.behaviour_mut().subscribe(&topic).unwrap();
    println!("Subscribed to topic successfully");

    let port = std::env::var("P2P_PORT").unwrap_or("40001".to_string());
    let addr: Multiaddr = format!("/ip4/0.0.0.0/tcp/{}", port).parse().unwrap();
    swarm.listen_on(addr).unwrap();

    // Dial S9 da VM o viceversa
    if port == "40001" {
        let s9_addr: Multiaddr = "/ip4/192.168.1.174/tcp/40003".parse().unwrap();
        if let Err(e) = swarm.dial(s9_addr.clone()) {
            println!("Dial failed: {:?}", e);
        } else {
            println!("Dialed S9 node successfully");
        }
    }

    // Crea un blocco simulato dopo un ritardo maggiore
    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    let block = vec![Transaction::new(
        "Node1".to_string(),
        "Node2".to_string(),
        10.0,
        &node1_key,
    )];
    println!("Simulated block created: {:?}", block);
    {
        let mut blockchain = blockchain.lock().await;
        if blockchain.add_block(block.clone(), "Node1".to_string()) {
            println!("Added simulated block locally: {:?}", blockchain.get_blocks());
        } else {
            println!("Failed to add simulated block locally");
        }
    }
    let message = serde_json::to_vec(&block).unwrap();
    if let Err(e) = swarm.behaviour_mut().publish(topic.clone(), message) {
        println!("Failed to publish message: {:?}", e);
    } else {
        println!("Published block: {:?}", block);
    }

    while let Some(event) = swarm.next().await {
        match event {
            SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                println!("Connection established with peer {:?}", peer_id);
            }
            SwarmEvent::Behaviour(gossipsub::Event::Message { message, .. }) => {
                println!("Received message: {:?}", message);
                if let Ok(transactions) = serde_json::from_slice::<Vec<Transaction>>(&message.data) {
                    let mut blockchain = blockchain.lock().await;
                    let all_valid = transactions.iter().all(|tx| {
                        if let Some(pub_key) = blockchain.public_keys.get(&tx.sender) {
                            let valid = tx.verify(pub_key);
                            println!("Verifying transaction from {}: valid={}", tx.sender, valid);
                            valid
                        } else {
                            println!("Public key for sender {} not found", tx.sender);
                            false
                        }
                    });
                    if all_valid {
                        blockchain.add_block(transactions.clone(), "Node1".to_string());
                        println!("Block added via P2P: {:?}", transactions);
                        println!("Current blockchain state after adding block: {:?}", blockchain.get_blocks());
                    } else {
                        println!("Invalid transaction signatures in block: {:?}", transactions);
                    }
                }
            }
            _ => {}
        }
    }
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let mut rng = OsRng;
    let alice_key = SigningKey::generate(&mut rng);
    let bob_key = SigningKey::generate(&mut rng);
    let charlie_key = SigningKey::generate(&mut rng);
    let node1_key = SigningKey::generate(&mut rng);

    println!("Chiave pubblica di Alice: {:?}", alice_key.verifying_key());
    println!("Chiave pubblica di Bob: {:?}", bob_key.verifying_key());
    println!("Chiave pubblica di Charlie: {:?}", charlie_key.verifying_key());
    println!("Chiave pubblica di Node1: {:?}", node1_key.verifying_key());
    println!();

    let mut blockchain = Blockchain::new("Node1".to_string());
    blockchain.add_public_key("Alice", alice_key.verifying_key());
    blockchain.add_public_key("Bob", bob_key.verifying_key());
    blockchain.add_public_key("Charlie", charlie_key.verifying_key());
    blockchain.add_public_key("Node1", node1_key.verifying_key());

    let tx1 = Transaction::new("Alice".to_string(), "Bob".to_string(), 30.0, &alice_key);
    blockchain.add_block(vec![tx1], "Node1".to_string());

    let tx2 = Transaction::new("Charlie".to_string(), "Alice".to_string(), 5.0, &charlie_key);
    blockchain.add_block(vec![tx2], "Node1".to_string());

    let static_path = std::env::var("STATIC_PATH").unwrap_or("static".to_string());
    let blockchain_state = Arc::new(Mutex::new(blockchain));
    tokio::spawn(run_p2p_node(blockchain_state.clone(), node1_key));

    rocket::build()
        .manage(blockchain_state)
        .mount("/", routes![get_blocks, get_balances, index])
        .mount("/static", FileServer::from(static_path))
        .configure(
            rocket::Config::figment()
                .merge(("address", "0.0.0.0"))
                .merge(("port", 8000)),
        )
        .launch()
        .await
        .map(|_| ())
}
