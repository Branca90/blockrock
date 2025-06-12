use libp2p::{
    identity,
    mdns::{Mdns, MdnsConfig},
    swarm::{Swarm, SwarmBuilder, SwarmEvent},
    NetworkBehaviour, PeerId,
};
use std::error::Error;

#[derive(NetworkBehaviour)]
pub struct ZionBehaviour {
    mdns: Mdns,
}

pub async fn start_p2p() -> Result<Swarm<ZionBehaviour>, Box<dyn Error>> {
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = local_key.public().to_peer_id();

    let transport = libp2p::tcp::TokioTcpConfig::new()
        .upgrade(libp2p::core::upgrade::Version::V1)
        .authenticate(libp2p::noise::NoiseConfig::xx(&local_key).unwrap())
        .multiplex(libp2p::yamux::YamuxConfig::default())
        .boxed();

    let mdns = Mdns::new(MdnsConfig::default()).await?;
    let behaviour = ZionBehaviour { mdns };

    let mut swarm = SwarmBuilder::new(transport, behaviour, local_peer_id)
        .executor(Box::new(|fut| {
            tokio::spawn(fut);
        }))
        .build();

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
    Ok(swarm)
}
