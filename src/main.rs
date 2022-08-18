use std::error::Error;

use libp2p::{futures::StreamExt, identity, mdns::*, swarm::SwarmEvent, Multiaddr, PeerId, Swarm};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let new_key = identity::Keypair::generate_ed25519();
    let new_peer_id = PeerId::from(new_key.public());
    println!("{:?}", new_peer_id);

    // let behaviour = /* DummyBehaviour::default(); */
    // Ping::new(PingConfig::new().with_keep_alive(true));
    let behaviour = Mdns::new(MdnsConfig::default()).await?;
    let transport = libp2p::development_transport(new_key).await?;
    let mut swarm = Swarm::new(transport, behaviour, new_peer_id);
    swarm.listen_on("/ip4/0.0.0.0/tcp/3000".parse()?)?;
    if let Some(remote_peer) = std::env::args().nth(1) {
        let remote_peer_multiaddr: Multiaddr = remote_peer.parse()?;
        swarm.dial(remote_peer_multiaddr)?;
        println!("Dialed remote peer:{:?}", remote_peer);
    }
    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("listen on local address{:?}", address)
            }
            SwarmEvent::Behaviour(MdnsEvent::Discovered(peers)) => {
                for (peer, addr) in peers {
                    println!("discovered :{:?}--{:?}", peer, addr)
                }
            }
            SwarmEvent::Behaviour(MdnsEvent::Expired(expired)) => {
                for (peer, addr) in expired {
                    println!("expired {} {}", peer, addr)
                }
            }
            _ => {}
        }
    }
}
