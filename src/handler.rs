use std::{
    collections::{HashMap, HashSet},
    net::SocketAddr,
    sync::MutexGuard,
};

use crate::{events::Event, Rooms, Sessions, Tx};
use futures_channel::mpsc::unbounded;
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};
use log::{info, warn};
use serde_json::json;
use tokio::net::TcpStream;
use tungstenite::Message;

pub async fn handle_connection(
    sessions: Sessions,
    rooms: Rooms,
    raw_stream: TcpStream,
    addr: SocketAddr,
) {
    info!("Incomming TCP connection from: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occured");
    info!("WebScoket connection established: {}", addr);

    let (tx, rx) = unbounded();
    sessions.lock().unwrap().insert(addr, tx);
    let (outgoing, incoming) = ws_stream.split();

    let broadcast_incoming = incoming.try_for_each(|msg| {
        info!("Received a message from {}", addr,);
        let mut rooms = rooms.lock().unwrap();
        let mut sessions = sessions.lock().unwrap();
        match msg.clone() {
            Message::Text(txt) => handle_event(txt, &mut rooms, &mut sessions, addr),
            Message::Close(_) => {
                // Should be removed from all rooms
                for (_, addresses) in rooms.iter_mut() {
                    if addresses.contains(&addr) {
                        addresses.remove(&addr);
                    }
                }
                info!("{addr} disconnected")
            }
            _ => {
                warn!("Uncatched message type")
            }
        }

        future::ok(())
    });

    let receive_from_others = rx.map(Ok).forward(outgoing);
    pin_mut!(broadcast_incoming, receive_from_others);
    future::select(broadcast_incoming, receive_from_others).await;

    warn!("{} disconnected", &addr);
    sessions.lock().unwrap().remove(&addr);
}

fn handle_event(
    msg: String,
    rooms: &mut MutexGuard<HashMap<String, HashSet<SocketAddr>>>,
    sessions: &mut MutexGuard<HashMap<SocketAddr, Tx>>,
    addr: SocketAddr,
) {
    let event: Event = serde_json::from_str(msg.as_str()).unwrap();

    info!("{:?}", event);

    match event {
        Event::Join { room } => match rooms.get_mut(&room) {
            Some(session) => {
                session.insert(addr);
            }
            None => {
                info!("Should create new room");
                let mut hs = HashSet::new();
                hs.insert(addr);
                rooms.insert(room, hs);
            }
        },
        Event::Message { to, msg } => {
            info!("{} is sending '{}' to {}", addr, msg, to);
            let sockets = rooms
                .iter()
                .find(|(r, _)| r == &&to)
                .map(|(_, sockets)| sockets);

            let message = serde_json::to_string_pretty(&json!({
                "message": msg
            })).unwrap();

            if let Some(sockets) = sockets {
                for socket in sockets.into_iter() {
                    if socket != &addr {
                        let ws_sink = sessions.get(socket).unwrap();
                        ws_sink.unbounded_send(Message::Text(message.clone())).unwrap();
                    }
                }
            }
            //      let boradcast_recipient = sessions
            //     .iter()
            //     .filter(|(peer_addr, _)| peer_addr != &&addr)
            //     .map(|(_, ws_sink)| ws_sink);

            // for recp in boradcast_recipient {
            //     // recp.unbounded_send(msg.clone()).unwrap();
            // }
        }
    }
}
