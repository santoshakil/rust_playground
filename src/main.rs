use futures::StreamExt;
use libp2p::{
    identify,
    swarm::{NetworkBehaviour, SwarmEvent},
    PeerId,
};
use std::error::Error;

#[derive(NetworkBehaviour)]
struct MyBehaviour {
    pub mdns: libp2p::mdns::tokio::Behaviour,
    pub identify: identify::Behaviour,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut swarm = build_swarm()?;

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    println!("Peer {:?}", swarm.local_peer_id());

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(e)) => match e {
                libp2p::mdns::Event::Discovered(list) => {
                    for (peer, addr) in list.clone() {
                        println!("Discovered {:?} {:?}", peer, addr);
                        swarm.dial(addr).unwrap();
                    }
                    // let peers = list.iter().map(|(peer, _)| peer.clone());
                    // swarm.behaviour_mut().identify.push(peers);
                }
                _ => {}
            },
            SwarmEvent::Behaviour(MyBehaviourEvent::Identify(e)) => match e {
                identify::Event::Received { peer_id, info } => {
                    println!("Received {:?} {:?}", peer_id, info);
                }
                identify::Event::Sent { peer_id } => {
                    println!("Sent {:?}", peer_id);
                }
                _ => {}
            },
            SwarmEvent::NewListenAddr { .. } => {}
            _ => {}
        }
    }
}

fn build_swarm() -> Result<libp2p::Swarm<MyBehaviour>, Box<dyn std::error::Error>> {
    let swarm = libp2p::SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            libp2p::tcp::Config::default(),
            libp2p::noise::Config::new,
            libp2p::yamux::Config::default,
        )?
        .with_behaviour(|key| {
            let peer_id = PeerId::from(key.public());

            let mdnsc = libp2p::mdns::Config::default();
            let mdns = libp2p::mdns::tokio::Behaviour::new(mdnsc, peer_id).unwrap();

            let mut identifyc = identify::Config::new("/ipfs/id/1.0.0".to_string(), key.public());
            identifyc.protocol_version = "ipfs/0.1.0 || Hello!!!".to_string();
            let identify = identify::Behaviour::new(identifyc);

            MyBehaviour { mdns, identify }
        })?
        .build();
    Ok(swarm)
}
