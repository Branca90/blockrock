mod p2p;

use rocket::{get, routes, State};
use rocket::serde::json::Json;
use tokio::sync::mpsc;
use tracing::{info, error};
use ed25519_dalek::{SigningKey, Signer};
use rand::rngs::OsRng;
use std::time::Duration;
use p2p::{P2PNode, P2PMessage, P2PCommand};

#[derive(serde::Serialize)]
struct HealthResponse {
    status: String,
}

#[derive(serde::Serialize)]
struct P2PStatusResponse {
    status: String,
}

#[derive(serde::Serialize)]
struct P2PPeersResponse {
    peers: Vec<String>,
}

#[get("/health")]
fn health() -> Json<HealthResponse> {
    info!("Richiesta a /health");
    Json(HealthResponse {
        status: "ok".to_string(),
    })
}

#[get("/")]
fn index() -> &'static str {
    "Zion Core sta forgiando verso la luce!"
}

#[get("/p2p/status")]
async fn p2p_status(state: &State<mpsc::Sender<P2PCommand>>) -> Json<P2PStatusResponse> {
    info!("Richiesta a /p2p/status");
    let (tx, mut rx) = mpsc::channel(1);
    if state.send(P2PCommand::GetPeerId { response: tx }).await.is_ok() {
        if let Some(peer_id) = rx.recv().await {
            Json(P2PStatusResponse {
                status: format!("peer_id: {}", peer_id),
            })
        } else {
            Json(P2PStatusResponse {
                status: "no response from node".to_string(),
            })
        }
    } else {
        Json(P2PStatusResponse {
            status: "failed to send command".to_string(),
        })
    }
}

#[get("/p2p/peers")]
async fn p2p_peers(state: &State<mpsc::Sender<P2PCommand>>) -> Json<P2PPeersResponse> {
    info!("Richiesta a /p2p/peers");
    let (tx, mut rx) = mpsc::channel(1);
    if state.send(P2PCommand::GetPeers { response: tx }).await.is_ok() {
        if let Some(peers) = rx.recv().await {
            let peers = peers.into_iter().map(|p| p.to_string()).collect();
            info!("Peer trovati: {:?}", peers);
            Json(P2PPeersResponse { peers })
        } else {
            Json(P2PPeersResponse { peers: vec![] })
        }
    } else {
        Json(P2PPeersResponse { peers: vec![] })
    }
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let mut csprng = OsRng {};
    let signing_key = SigningKey::generate(&mut csprng);

    let (command_tx, command_rx) = mpsc::channel(32);
    let mut node = P2PNode::new(command_rx).await?;
    node.listen("/ip4/0.0.0.0/tcp/0").await?;

    tokio::spawn(async move {
        if let Err(e) = node.run().await {
            error!("Errore nel run del nodo P2P: {}", e);
        }
    });

    let command_tx_clone = command_tx.clone();
    let signing_key_clone = signing_key.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(10));
        loop {
            interval.tick().await;
            let message_data = b"Ciao da Zion Core!";
            let signature = signing_key_clone.sign(message_data);
            let p2p_message = P2PMessage {
                sender: "Nodo1".to_string(),
                data: message_data.to_vec(),
                signature: signature.to_bytes().to_vec(),
            };
            if command_tx_clone
                .send(P2PCommand::PublishMessage(p2p_message))
                .await
                .is_err()
            {
                error!("Errore nell'invio del comando di pubblicazione");
            } else {
                info!("Comando di pubblicazione inviato!");
            }
        }
    });

    let port = std::env::var("ROCKET_PORT").unwrap_or("8000".to_string());
    let config = rocket::Config {
        port: port.parse::<u16>().unwrap_or(8000),
        ..rocket::Config::debug_default()
    };

    rocket::build()
        .configure(config)
        .manage(command_tx)
        .mount("/", routes![health, index, p2p_status, p2p_peers])
        .launch()
        .await?;

    Ok(())
}
