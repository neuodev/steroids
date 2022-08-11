mod events;
mod handler;
mod db;

use std::{
    collections::{HashMap, HashSet},
    env,
    io::Error as IoError,
    net::SocketAddr,
    sync::{Arc, Mutex}, process,
};

use env_logger;
use futures_channel::mpsc::UnboundedSender;
use log::{info, error};
use tokio::net::TcpListener;
use tungstenite::protocol::Message;
use tokio_postgres::{Client, NoTls};
use handler::handle_connection;
use clap::Parser;

use crate::db::migrate;

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

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser, default_value_t=false)]
    migrate: bool,
    #[clap(long, value_parser, default_value = "0.0.0.0:8080")]
    addr: String,
}

#[tokio::main]
async fn main() -> Result<(), IoError> {
    env_logger::init();
    let args = Args::parse();
    let addr = args.addr;

    let sessions: Sessions = Arc::new(Mutex::new(HashMap::new()));
    let rooms: Rooms = Arc::new(Mutex::new(HashMap::new()));
    let (client, connection) = tokio_postgres::connect("postgresql://postgres:changeme@localhost/chat", NoTls).await.unwrap();
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            error!("Database connection error: {e}");
            process::exit(1);
        }
    });
    let client = Arc::new(Mutex::new(client));
    // Create database tables
    if args.migrate == true {
        migrate(client.clone()).await.unwrap_or_else(|e| {
            error!("Unable to migrate: {e}");
            process::exit(1);
        });

        info!("Migrated successfully");
    }

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
