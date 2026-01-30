use std::time::{Duration, SystemTime, UNIX_EPOCH};

use eframe::egui::{self, Color32, FontFamily, FontId, FontSelection, Rgba, RichText, Stroke, Style, TextFormat, text::LayoutJob};
use egui_twemoji::EmojiLabel;
use tokio_tungstenite::tungstenite::{Utf8Bytes, protocol::Message};

use shared::{Color, Message as NagaMessage, User};

use crate::{Nagger, emoji, keyboard, network::{ClientCommand, ClientEvent}};

impl Nagger {
    pub fn draw_ui(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());
        ctx.request_repaint_after(Duration::from_millis(16));

        egui::CentralPanel::default()
            .frame(egui::Frame::new().outer_margin(egui::Margin::symmetric(15, 5)))
            .show(ctx, |ui| {
            if self.connected {
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
                    ui.horizontal(|ui| {
                        let response = ui.add(
                            egui::TextEdit::singleline(&mut self.message)
                            .hint_text("Nag")
                        );
        
                        if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) && self.message != String::new() {
                            let message = NagaMessage {
                                text: self.message.clone(),
                                user: self.user.clone(),
                            };
        
                            let message_event = shared::Event {
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

                        if self.hook_enabled {
                            response.request_focus();
                        }
    
                        let response;
                        if self.chat_enabled {
                            response = ui.button("Enabled");
                        } else {
                            response = ui.button("Disabled");
                        }
    
                        if response.clicked() {
                            self.chat_enabled = !self.chat_enabled;
                            set_enabled(self.chat_enabled);
                        }
                    });

                    ui.separator();

                    self.draw_emoji(ctx, _frame, ui);

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
        
                                        ui.horizontal(|ui| {
                                            ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);
                                            ui.label(RichText::new(user.username.clone()).color(color).size(14.0));

                                            if user.system {
                                                EmojiLabel::new(
                                                    RichText::new(&(String::from(" >> ") + &msg.text)).color(Color32::WHITE).size(14.0)
                                                ).show(ui);
                                            } else {
                                                EmojiLabel::new(
                                                    RichText::new(&(String::from(": ") + &msg.text)).color(Color32::WHITE).size(14.0)
                                                ).show(ui);
                                            }
                                        });
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
                    let mut address = self.address.clone();
                    if address.is_empty() {
                        address = String::from("ws://127.0.0.1:6967");
                    }

                    tokio::spawn(async move {
                        let _ = tx.send(ClientCommand::Connect(address)).await;
                    });
                }

                match self.rx.try_recv() {
                    Ok(ClientEvent::Connected) => {
                        self.connected = true;
                        let connected_event = shared::Event {
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
                        let event = shared::Event {
                            event_type: shared::EventType::Message(m),
                            time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis(),
                        };
                        events.push(event);
    
                        let tx = self.tx.clone();
                        tokio::spawn(async move {
                            tx.send(ClientCommand::Send(msg)).await.unwrap();
                        });
                    },
                    _ => {}
                }
            }
            
       });

    }

    fn draw_emoji(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame, ui: &mut egui::Ui ) {
        if !self.message.contains(":") { return; }
        let string_iter = self.message.split(":");
        if string_iter.clone().count() < 2 { return; }

        let query = string_iter.last().unwrap();
        let results = emoji::search_emojis_by_name(&self.emojis, query);

        if ui.input(|i| i.key_pressed(egui::Key::Tab)) && results.len() > 0 {
            // replace emoji text with emoji
            let start = self.message.find(":").unwrap();
            self.message.replace_range(start..self.message.len(), "");

            let (_, emoji, _) = results.first().unwrap();

            self.message.push_str(emoji);
        }

        egui::frame::Frame::new()
            .corner_radius(0)
            .inner_margin(5)
            .outer_margin(5)
            .fill(Color32::BLACK)
            .stroke(Stroke::new(1.0, Color32::GRAY))
            .show(ui, |ui| {
                let mut idx = 0;
                for (name, result, _) in results {
                    if idx >= 5 { break; }

                    ui.horizontal(|ui| {
                        EmojiLabel::new(result).show(ui);
                        ui.separator();
                        ui.label(name);
                    });

                    idx += 1;
                }
            });

    }
}

fn set_enabled(enabled: bool) {
    #[cfg(target_os = "windows")]
    <keyboard::windows_hook::WindowsKeyboardHook as keyboard::KeyboardHook>::set_chat_enabled(enabled);
    #[cfg(target_os = "linux")]
    <keyboard::linux_hook::LinuxKeyboardHook as keyboard::KeyboardHook>::set_chat_enabled(enabled);
}