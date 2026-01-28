//! A simple example of hooking up stdin/stdout to a WebSocket stream.
//!
//! This example will connect to a server specified in the argument list and
//! then forward all data read on stdin to the server, printing out all data
//! received on stdout.
//!
//! Note that this is not currently optimized for performance, especially around
//! buffer management. Rather it's intended to show an example of working with a
//! client.
//!
//! You can use this example together with the `server` example.

use std::{sync::{Arc}, time::{SystemTime, UNIX_EPOCH}};

use eframe::egui::{self, Color32, FontFamily, FontId, FontSelection, Rgba, RichText, Style, TextFormat, Vec2, text::LayoutJob};
use std::sync::Mutex;
use tokio_tungstenite::tungstenite::{Utf8Bytes, protocol::Message};

use shared::{Color, Event, Message as NagaMessage, User};

use crate::network::{ClientCommand, ClientEvent, Network};

mod network;

struct Nagger {
    user: User,
    address: String,
    message: String,
    events: Arc<Mutex<Vec<Event>>>,
    connected: bool,
    rx: tokio::sync::mpsc::Receiver<ClientEvent>,
    tx: tokio::sync::mpsc::Sender<ClientCommand>,
}

impl Nagger {
    fn new(
        cc: &eframe::CreationContext<'_>,
        events: Arc<Mutex<Vec<Event>>>,
        rx: tokio::sync::mpsc::Receiver<ClientEvent>,
        tx: tokio::sync::mpsc::Sender<ClientCommand>,
        mut msg_rx: tokio::sync::mpsc::Receiver<ClientEvent>,
    ) -> Self {
        // cc.storage.unwrap().flush();
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let user = User {
            username: String::new(),
            color: Color{ r: 255, g: 255, b: 255},
            system: false,
        };

        let ctx = cc.egui_ctx.clone();
        tokio::spawn(async move {
            loop {
                match msg_rx.recv().await {
                    Some(a) => match a {
                        ClientEvent::Message => {
                            ctx.request_repaint();
                        },
                        _ => {}
                    },
                    _ => {},
                }
            }
        });

        Self {
            user,
            address: String::new(),
            message: String::new(),
            events,
            connected: false,
            rx,
            tx,
        }
    }
}

impl eframe::App for Nagger {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.connected {
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
                    let response = ui.add(
                        egui::TextEdit::singleline(&mut self.message)
                        .hint_text("Nag")
                    );
    
                    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        let message = NagaMessage {
                            text: self.message.clone(),
                            user: self.user.clone(),
                        };
    
                        let message_event = Event {
                            event_type: shared::EventType::Message(message),
                            time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis(),
                        };

                        
                        let string = serde_json::to_string(&message_event).unwrap();
                        let payload = Utf8Bytes::from(string);
                        let msg = Message::Text(payload);
                        
                        let tx = self.tx.clone();
                        tokio::spawn(async move {
                            tx.send(ClientCommand::Send(msg)).await.unwrap();
                        });
                        self.message = String::new();
                        self.events.lock().unwrap().push(message_event);
                    }

                    ui.separator();

                    egui::ScrollArea::vertical()
                        .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
                        .scroll([false, true])
                        // .scroll_offset(Vec2::new(0.0, -1.0))
                        .auto_shrink([false, false])
                        .stick_to_bottom(true)
                        .show(ui, |ui| {
                        ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                            let events = self.events.lock().unwrap();
                            for event in events.iter() {
                                match &event.event_type {
                                    shared::EventType::Message(msg) => {
                                        let user = &msg.user;
                                        let color = Rgba::from_rgb(
                                            user.color.r as f32 / 255.0, 
                                            user.color.g as f32 / 255.0, 
                                            user.color.b as f32 / 255.0
                                        );
        
                                        let style = Style::default();
                                        let mut job = LayoutJob::default();
                                        RichText::new(user.username.clone()).color(color)
                                            .append_to(
                                                &mut job, 
                                                &style, 
                                                FontSelection::Default, 
                                                egui::Align::Min
                                        );
        
                                        if user.system {
                                            job.append(
                                                &(String::from(" >> ") + &msg.text),
                                                0.0,
                                                TextFormat {
                                                    font_id: FontId::new(14.0, FontFamily::Proportional),
                                                    color: Color32::WHITE,
                                                    ..Default::default()
                                                },
                                            );
                                        } else {
                                            job.append(
                                                &(String::from(": ") + &msg.text),
                                                0.0,
                                                TextFormat {
                                                    font_id: FontId::new(14.0, FontFamily::Proportional),
                                                    color: Color32::WHITE,
                                                    ..Default::default()
                                                },
                                            );
                                        }
        
                                        ui.label(job);
                                    },
                                    shared::EventType::Joined(user) => {
                                        let color = Rgba::from_rgb(
                                            user.color.r as f32 / 255.0, 
                                            user.color.g as f32 / 255.0, 
                                            user.color.b as f32 / 255.0
                                        );
        
                                        let style = Style::default();
                                        let mut job = LayoutJob::default();
                                        RichText::new(user.username.clone()).color(color)
                                            .append_to(
                                                &mut job, 
                                                &style, 
                                                FontSelection::Default, 
                                                egui::Align::Min
                                            );
                                        job.append(
                                            &(String::from("joined!")),
                                            4.0,
                                            TextFormat {
                                                font_id: FontId::new(14.0, FontFamily::Proportional),
                                                color: Color32::WHITE,
                                                ..Default::default()
                                            },
                                        );
        
                                        ui.label(job);
                                    },
                                    shared::EventType::Left(user) => {
                                        let color = Rgba::from_rgb(
                                            user.color.r as f32 / 255.0, 
                                            user.color.g as f32 / 255.0, 
                                            user.color.b as f32 / 255.0
                                        );
        
                                        let style = Style::default();
                                        let mut job = LayoutJob::default();
                                        RichText::new(user.username.clone()).color(color)
                                            .append_to(
                                                &mut job, 
                                                &style, 
                                                FontSelection::Default, 
                                                egui::Align::Min
                                            );
                                        job.append(
                                            &(String::from("left!")),
                                            4.0,
                                            TextFormat {
                                                font_id: FontId::new(14.0, FontFamily::Proportional),
                                                color: Color32::WHITE,
                                                ..Default::default()
                                            },
                                        );
        
                                        ui.label(job);
                                    }
                                }
                            }
                        });
                    });
                });
            } else {
                ui.horizontal(|ui| {
                    ui.label("Username");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.user.username)
                        .hint_text("Nagger")
                    );

                });
                ui.horizontal(|ui| {
                    ui.label("Url");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.address)
                        .hint_text("ws://[server.served]:[port]")
                    );
                });

                ui.separator();

                if ui.button("Connect").clicked() {
                    let tx = self.tx.clone();
                    let address = self.address.clone();
                    tokio::spawn(async move {
                        let _ = tx.send(ClientCommand::Connect(address)).await;
                    });
                }

                match self.rx.try_recv() {
                    Ok(ClientEvent::Connected) => {
                        self.connected = true;
                        let connected_event = Event {
                            event_type: shared::EventType::Joined(self.user.clone()),
                            time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis(),
                        };
    
                        let string = serde_json::to_string(&connected_event).unwrap();
                        let payload = Utf8Bytes::from(string);
                        let msg = Message::Text(payload);
    
                        // add hello msg
                        let mut events = self.events.lock().unwrap();
                        let user = User { username: String::from("SYS"), color: Color { r: 255, g: 0, b: 0 }, system: true };
                        let m = NagaMessage { text: format!("Hello {}!", self.user.username), user };
                        let event = Event {
                            event_type: shared::EventType::Message(m),
                            time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis(),
                        };
                        events.push(event);
    
                        let tx = self.tx.clone();
                        tokio::spawn(async move {
                            tx.send(ClientCommand::Send(msg)).await.unwrap();
                        });
                    },
                    Ok(ClientEvent::Message) => {
                        ctx.request_repaint();
                    }
                    _ => {}
                }
            }
            
       });

    }
}

#[tokio::main]
async fn main() {
    let events = Arc::new(Mutex::new(Vec::new()));

    let (cmd_tx, cmd_rx) = tokio::sync::mpsc::channel::<ClientCommand>(10);
    let (ev_tx, ev_rx) = tokio::sync::mpsc::channel::<ClientEvent>(10);
    let (msg_tx, msg_rx) = tokio::sync::mpsc::channel::<ClientEvent>(10);

    let network = Network::new(events.clone(), ev_tx, cmd_rx, msg_tx);
    network.run();

    let icon_data = eframe::icon_data::from_png_bytes(include_bytes!("../../assets/naga.png"))
        .expect("Failed to load icon data");
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_icon(icon_data),
        ..Default::default()
    };

    eframe::run_native("Nagger", native_options, Box::new(|cc| Ok(Box::new(Nagger::new(cc, events.clone(), ev_rx, cmd_tx, msg_rx))))).unwrap();
}