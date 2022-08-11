mod events;
mod handler;

use std::{
    collections::{HashMap, HashSet},
    env,
    io::Error as IoError,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use env_logger;
use futures_channel::mpsc::UnboundedSender;
use log::info;
use tokio::net::TcpListener;
use tungstenite::protocol::Message;

use handler::handle_connection;

pub type Tx = UnboundedSender<Message>;
pub type RoomId = String;
pub type Sessions = Arc<Mutex<HashMap<SocketAddr, Tx>>>;
pub type Rooms = Arc<Mutex<HashMap<RoomId, HashSet<SocketAddr>>>>;

#[tokio::main]
async fn main() -> Result<(), IoError> {
    env_logger::init();

    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "0.0.0.0:8080".to_string());

    let sessions: Sessions = Arc::new(Mutex::new(HashMap::new()));
    let rooms: Rooms = Arc::new(Mutex::new(HashMap::new()));

    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");
    info!("Listening on: {}", addr);

    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(
            sessions.clone(),
            rooms.clone(),
            stream,
            addr,
        ));
    }

    Ok(())
}
