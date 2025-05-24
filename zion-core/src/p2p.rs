use libp2p::{gossipsub, identity, swarm::Swarm, PeerId};
use ed25519_dalek::SigningKey;

pub struct P2PNode {
    swarm: Swarm<gossipsub::Behaviour>,
    peer_id: PeerId,
}

impl P2PNode {
    pub fn new(key: SigningKey) -> Self {
        // Da implementare: configurazione swarm
        todo!()
    }
}
