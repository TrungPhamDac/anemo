use anemo::{types::PeerEvent, Network};
use anemo_tower::trace::TraceLayer;
use chat_server::{HelloClient, HelloServer, HelloRequest, MyHello};
use tower::Layer;
use tracing::{info, Level};
// use std::io::stdin;
use std::net::SocketAddr;

fn set_up_logs() {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}

#[derive(Copy, Clone)]
pub struct KnownPeerConfig {
    pub addr: SocketAddr,
}


#[tokio::main]
async fn main() {
    set_up_logs();
    // let mut port = String::new();
    // println!("port: ");
    // stdin().read_line(&mut port).expect("Fail to read");

    let network_local = format!("localhost:6002");
    println!("{}", network_local);
    let network = Network::bind(network_local.to_owned())
        .private_key(random_key())
        .server_name("test")
        .start(TraceLayer::new_for_server_errors()
        .layer(HelloServer::new(MyHello::default())))
        .unwrap();
    let network_addr = network.local_addr();
    println!("{}", network_addr);
    loop{
        // let mut addr = String::new();
        // println!("Address: ");
        // stdin().read_line(&mut addr).expect("Fail to read");



        // SocketAddr::new(self.ip.parse().unwrap(), *port);

        let peer = network.connect("[::1]:6004").await.unwrap();

        let peer = network.peer(peer).unwrap();
        let mut client = HelloClient::new(peer);
        let response = client
        .say_hello(HelloRequest {
            name: "Brandon".into(),
        })
        .await
        .unwrap();

        info!("{:#?}", response);
    }
}


fn random_key() -> [u8; 32] {
    let mut rng = rand::thread_rng();
    let mut bytes = [0u8; 32];
    rand::RngCore::fill_bytes(&mut rng, &mut bytes[..]);
    bytes
}
