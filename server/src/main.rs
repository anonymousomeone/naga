//! A chat server that broadcasts a message to all connections.
//!
//! This is a simple line-based server which accepts WebSocket connections,
//! reads lines from those connections, and broadcasts the lines to all other
//! connected clients.
//!
//! You can test this out by running:
//!
//!     cargo run --example server 127.0.0.1:12345
//!
//! And then in another window run:
//!
//!     cargo run --example client ws://127.0.0.1:12345/
//!
//! You can run the second command in multiple windows and then chat between the
//! two, seeing the messages from the other client as they're received. For all
//! connected clients they'll all join the same room and see everyone else's
//! messages.

use std::{
    collections::HashMap,
    env,
    io::Error as IoError,
    net::SocketAddr,
    sync::{Arc, Mutex}, time::{SystemTime, UNIX_EPOCH},
};

use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};

use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::{Utf8Bytes, protocol::Message};

use shared::User;
use shared::Event;
use serde_json;

type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;
type Clients = Arc<Mutex<HashMap<SocketAddr, User>>>;

async fn handle_connection(peer_map: PeerMap, clients: Clients, raw_stream: TcpStream, addr: SocketAddr) {
    println!("Incoming TCP connection from: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);

    // insert this connection to peer map
    let (tx, rx) = unbounded();
    peer_map.lock().unwrap().insert(addr, tx);

    let (outgoing, incoming) = ws_stream.split();

    // handle incoming msg from peer
    let broadcast_incoming = incoming.try_for_each(|msg| {
        println!("Received a message from {}: {}", addr, msg.to_text().unwrap());
        let event = serde_json::from_str::<Event>(msg.to_text().unwrap()).unwrap();

        match event.event_type {
            shared::EventType::Joined(mut user) => {
                println!("{} upgraded to client", addr);
                user.system = false;
                clients.lock().unwrap().insert(addr, user);
            },
            _ => {}
        }

        let peers = peer_map.lock().unwrap();

        // send to everyone except sender
        let broadcast_recipients =
            peers.iter().filter(|(peer_addr, _)| peer_addr != &&addr).map(|(_, ws_sink)| ws_sink);

        for recp in broadcast_recipients {
            recp.unbounded_send(msg.clone()).unwrap();
        }

        future::ok(())
    });

    let receive_from_others = rx.map(Ok).forward(outgoing);

    pin_mut!(broadcast_incoming, receive_from_others);
    future::select(broadcast_incoming, receive_from_others).await;

    // client disconnected, @everyone
    println!("{} disconnected", &addr);
    let mut peers = peer_map.lock().unwrap();
    peers.remove(&addr);

    let disconnect_event = Event {
        event_type: shared::EventType::Left(clients.lock().unwrap().get(&addr).expect("No user existed").clone()),
        time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis(),
    };

    let string = serde_json::to_string(&disconnect_event).unwrap();
    let payload = Utf8Bytes::from(string);
    let msg = Message::Text(payload);
    for recp in peers.iter().map(|(_, ws_sink)| ws_sink) {
        recp.unbounded_send(msg.clone()).unwrap();
    }

    let mut clients = clients.lock().unwrap();
    clients.remove(&addr);
}

#[tokio::main]
async fn main() -> Result<(), IoError> {
    let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:6967".to_string());

    let peers = PeerMap::new(Mutex::new(HashMap::new()));
    let clients = Clients::new(Mutex::new(HashMap::new()));
    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    // Let's spawn the handling of each connection in a separate task.
    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(peers.clone(), clients.clone(), stream, addr));
    }

    Ok(())
}
