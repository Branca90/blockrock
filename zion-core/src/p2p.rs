use libp2p::{
    futures::StreamExt,
    gossipsub::{self, MessageId},
    identity,
    mdns::{self, tokio::Behaviour as MdnsBehaviour},
    noise,
    swarm::{Swarm, SwarmEvent, Config},
    tcp,
    yamux,
    PeerId,
    Transport,
};
use libp2p_swarm_derive::NetworkBehaviour;
use sha2::{Digest, Sha256};
use std::error::Error;
use std::time::Duration;
use tokio::sync::mpsc;
use tracing::{info, warn, error};
use serde::{Deserialize, Serialize};

// Struct per i messaggi P2P
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct P2PMessage {
    pub sender: String,
    pub data: Vec<u8>,
    pub signature: Vec<u8>,
}

// Comandi per il nodo P2P
#[derive(Debug)]
pub enum P2PCommand {
    PublishMessage(P2PMessage),
    GetPeerId { response: mpsc::Sender<PeerId> },
    GetPeers { response: mpsc::Sender<Vec<PeerId>> },
}

// Struct per il comportamento combinato di Gossipsub e mDNS
#[derive(NetworkBehaviour)]
#[behaviour(out_event = "P2PEvent")]
struct P2PBehaviour {
    gossipsub: gossipsub::Behaviour,
    mdns: MdnsBehaviour,
}

#[derive(Debug)]
enum P2PEvent {
    Gossipsub(gossipsub::Event),
    Mdns(mdns::Event),
}

impl From<gossipsub::Event> for P2PEvent {
    fn from(event: gossipsub::Event) -> Self {
        P2PEvent::Gossipsub(event)
    }
}

impl From<mdns::Event> for P2PEvent {
    fn from(event: mdns::Event) -> Self {
        P2PEvent::Mdns(event)
    }
}

// Struct principale del nodo P2P
pub struct P2PNode {
    swarm: Swarm<P2PBehaviour>,
    peer_id: PeerId,
    topic: gossipsub::IdentTopic,
    command_rx: mpsc::Receiver<P2PCommand>,
}

impl P2PNode {
    pub async fn new(command_rx: mpsc::Receiver<P2PCommand>) -> Result<Self, Box<dyn Error>> {
        let local_key = identity::Keypair::generate_ed25519();
        let peer_id = PeerId::from(local_key.public());
        info!("Nuovo nodo creato con peer_id: {}", peer_id);

        let gossipsub_config = gossipsub::ConfigBuilder::default()
            .validation_mode(gossipsub::ValidationMode::Strict)
            .message_id_fn(|msg| {
                let mut hasher = Sha256::new();
                hasher.update(&msg.data);
                MessageId::from(hasher.finalize().to_vec())
            })
            .heartbeat_interval(Duration::from_secs(5))
            .build()?;

        let gossipsub = gossipsub::Behaviour::new(
            gossipsub::MessageAuthenticity::Signed(local_key.clone()),
            gossipsub_config,
        )?;

        let mdns = MdnsBehaviour::new(mdns::Config::default(), peer_id)?;

        let transport = tcp::tokio::Transport::new(tcp::Config::default().nodelay(true))
            .upgrade(libp2p::core::upgrade::Version::V1)
            .authenticate(noise::Config::new(&local_key)?)
            .multiplex(yamux::Config::default())
            .boxed();

        let behaviour = P2PBehaviour { gossipsub, mdns };

        let swarm_config = Config::with_tokio_executor()
            .with_idle_connection_timeout(Duration::from_secs(120));

        let mut swarm = Swarm::new(transport, behaviour, peer_id, swarm_config);

        let topic = gossipsub::IdentTopic::new("zion-core-topic");
        swarm.behaviour_mut().gossipsub.subscribe(&topic)?;
        info!("Sottoscritto al topic: {}", topic);

        Ok(P2PNode {
            swarm,
            peer_id,
            topic,
            command_rx,
        })
    }

    pub async fn listen(&mut self, addr: &str) -> Result<(), Box<dyn Error>> {
        self.swarm.listen_on(addr.parse()?)?;
        Ok(())
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            tokio::select! {
                event = self.swarm.select_next_some() => {
                    match event {
                        SwarmEvent::Behaviour(P2PEvent::Gossipsub(gossipsub::Event::Message { message, .. })) => {
                            match serde_json::from_slice::<P2PMessage>(&message.data) {
                                Ok(p2p_message) => {
                                    info!("Messaggio ricevuto da {}: {:?}", p2p_message.sender, p2p_message.data);
                                }
                                Err(e) => {
                                    warn!("Impossibile deserializzare il messaggio: {}", e);
                                }
                            }
                        }
                        SwarmEvent::Behaviour(P2PEvent::Mdns(mdns::Event::Discovered(peers))) => {
                            for (peer_id, addr) in peers {
                                info!("Scoperto peer: {} su {}", peer_id, addr);
                                self.swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                                if let Err(e) = self.swarm.dial(addr) {
                                    error!("Errore nel dial: {}", e);
                                }
                            }
                        }
                        SwarmEvent::Behaviour(P2PEvent::Mdns(mdns::Event::Expired(peers))) => {
                            for (peer_id, addr) in peers {
                                info!("Peer scaduto: {} su {}", peer_id, addr);
                                self.swarm.behaviour_mut().gossipsub.remove_explicit_peer(&peer_id);
                            }
                        }
                        SwarmEvent::NewListenAddr { address, .. } => {
                            info!("Ascolto su: {}", address);
                        }
                        SwarmEvent::ConnectionEstablished { peer_id, endpoint, .. } => {
                            info!("Connesso a: {} (endpoint: {:?})", peer_id, endpoint);
                        }
                        SwarmEvent::ConnectionClosed { peer_id, cause, .. } => {
                            info!("Disconnesso da: {} (causa: {:?})", peer_id, cause);
                        }
                        SwarmEvent::OutgoingConnectionError { peer_id, error, .. } => {
                            error!("Errore connessione in uscita a {:?}: {}", peer_id, error);
                        }
                        _ => {}
                    }
                }
                Some(command) = self.command_rx.recv() => {
                    match command {
                        P2PCommand::PublishMessage(message) => {
                            let data = match serde_json::to_vec(&message) {
                                Ok(data) => data,
                                Err(e) => {
                                    error!("Errore nella serializzazione del messaggio: {}", e);
                                    continue;
                                }
                            };
                            info!("Tentativo di pubblicazione messaggio: {:?}", message);
                            if let Err(e) = self.swarm.behaviour_mut().gossipsub.publish(self.topic.clone(), data) {
                                error!("Errore nella pubblicazione del messaggio: {}", e);
                            } else {
                                info!("Messaggio inviato!");
                            }
                        }
                        P2PCommand::GetPeerId { response } => {
                            let _ = response.send(self.peer_id).await;
                        }
                        P2PCommand::GetPeers { response } => {
                            let peers = self.swarm.connected_peers().copied().collect::<Vec<_>>();
                            let _ = response.send(peers).await;
                        }
                    }
                }
            }
        }
    }
}
