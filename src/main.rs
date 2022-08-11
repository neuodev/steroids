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
use tokio_postgres::{Client, NoTls, Error};
use handler::handle_connection;

pub type Tx = UnboundedSender<Message>;
pub type RoomId = String;
pub type Sessions = Arc<Mutex<HashMap<SocketAddr, Tx>>>;
pub type Rooms = Arc<Mutex<HashMap<RoomId, HashSet<SocketAddr>>>>;
pub type Database = Arc<Mutex<Client>>;

pub struct AppState {
    sessions: Sessions,
    rooms: Rooms,
    db: Database,
}

#[tokio::main]
async fn main() -> Result<(), IoError> {
    env_logger::init();

    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "0.0.0.0:8080".to_string());

    let sessions: Sessions = Arc::new(Mutex::new(HashMap::new()));
    let rooms: Rooms = Arc::new(Mutex::new(HashMap::new()));
    let (client, connection) = tokio_postgres::connect("postgresql://postgres:changeme@localhost/chat", NoTls).await.unwrap();
    let client = Arc::new(Mutex::new(client));
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");
    info!("Listening on: {}", addr);

    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(
            AppState {
                sessions: sessions.clone(),
                rooms: rooms.clone(),
                db: client.clone(),
            },
            stream,
            addr,
        ));
    }

    Ok(())
}
