use std::sync::Arc;

use futures_util::stream::SplitSink;
use futures_util::sink::SinkExt;
use futures_util::StreamExt;
use shared::Event;
use tokio::net::TcpStream;
use std::sync::Mutex;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async, tungstenite::Message};

pub struct Network {
    writer: Option<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>,
    // reader: Option<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>,
    events: Arc<Mutex<Vec<Event>>>,
    tx: tokio::sync::mpsc::Sender<ClientEvent>,
    rx: tokio::sync::mpsc::Receiver<ClientCommand>,
    msg_tx: tokio::sync::mpsc::Sender<ClientEvent>,
}

pub enum ClientCommand {
    Send(Message),
    Connect(String),
}

pub enum ClientEvent {
    Connected,
    Message
}

impl Network {
    pub fn new(
        events: Arc<Mutex<Vec<Event>>>,
        tx: tokio::sync::mpsc::Sender<ClientEvent>,
        rx: tokio::sync::mpsc::Receiver<ClientCommand>,
        msg_tx: tokio::sync::mpsc::Sender<ClientEvent>,
    ) -> Self {
        Network {
            writer: None,
            events,
            rx,
            tx,
            msg_tx,
        }
    }

    pub fn run(mut self) {
        tokio::spawn(async move {

            while let Some(cmd) = self.rx.recv().await {
                match cmd {
                    ClientCommand::Send(msg) => {
                        self.writer.as_mut().unwrap().send(msg).await.ok();
                    },
                    ClientCommand::Connect(addr) => {
                        println!("The Bluetooth device is ready to pair");
                        let (ws_stream, _) = connect_async(addr).await.expect("Failed to connect");
                        let (write, read) = ws_stream.split();

                        self.writer = Some(write);

                        let events = self.events.clone();
                        let msg_tx = self.msg_tx.clone();
                        tokio::spawn(async move {
                            read.for_each(|message| async {
                                let event = serde_json::from_str::<Event>(&message.unwrap().into_text().unwrap()).unwrap();
                                
                                // tokio::io::stdout().write_all(&data).await.unwrap();
                                println!("{:?}", event);
                                events.lock().unwrap().push(event);
                                msg_tx.send(ClientEvent::Message).await.unwrap();
                            }).await;
                        });
                        println!("The Bluetooth device is connected uh successfully");
                        self.tx.send(ClientEvent::Connected).await.unwrap();
                    },
                }
            }
        });
    }
}