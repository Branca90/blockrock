use libp2p::{
    core::upgrade,
    identity,
    mdns::{tokio::Behaviour as Mdns, Config as MdnsConfig, Event as MdnsEvent},
    noise,
    Swarm, SwarmBuilder,
    tcp,
    yamux,
    PeerId,
    Transport,
};
use libp2p_swarm_derive::NetworkBehaviour;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;
use blockrock_core::blockchain::Blockchain;

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "CustomEvent")]
pub struct MyBehaviour {
    mdns: Mdns,
}

#[derive(Debug)]
pub enum CustomEvent {
    Mdns(MdnsEvent),
}

impl From<MdnsEvent> for CustomEvent {
    fn from(event: MdnsEvent) -> Self {
        CustomEvent::Mdns(event)
    }
}

impl MyBehaviour {
    pub async fn new(local_peer_id: PeerId, _blockchain: Arc<Mutex<Blockchain>>) -> Result<Self, Box<dyn Error>> {
        let mdns = Mdns::new(MdnsConfig::default(), local_peer_id)?;
        Ok(MyBehaviour { mdns })
    }
}

pub async fn start_p2p_node(blockchain: Arc<Mutex<Blockchain>>) -> Result<Swarm<MyBehaviour>, Box<dyn Error>> {
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("Local peer id: {:?}", local_peer_id);

    let _transport = tcp::tokio::Transport::default()
        .upgrade(upgrade::Version::V1)
        .authenticate(noise::Config::new(&local_key)?)
        .multiplex(yamux::Config::default())
        .boxed();

    let behaviour = MyBehaviour::new(local_peer_id, blockchain).await?;
    let mut swarm = SwarmBuilder::with_existing_identity(local_key)
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            || yamux::Config::default()
        )?
        .with_behaviour(|_| Ok(behaviour))?
        .build();

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    Ok(swarm)
}
