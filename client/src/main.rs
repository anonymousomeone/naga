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

use std::{collections::HashMap, sync::Arc};

use eframe::egui;
use eframe::egui::Key;
use std::sync::Mutex;

use shared::{Color, User};

use crate::{keyboard::{Keyboard, KeyboardHook}, network::{ClientCommand, ClientEvent, Network}};

mod network;
mod keyboard;
mod ui;
mod emoji;

struct Nagger {
    user: User,
    address: String,
    message: String,
    events: Arc<Mutex<Vec<shared::Event>>>,
    emojis: HashMap<String, String>,
    connected: bool,
    hook_enabled: bool,
    chat_enabled: bool,
    keyboard: Keyboard,
    rx: tokio::sync::mpsc::Receiver<ClientEvent>,
    tx: tokio::sync::mpsc::Sender<ClientCommand>,
}

impl Nagger {
    fn new(
        cc: &eframe::CreationContext<'_>,
        events: Arc<Mutex<Vec<shared::Event>>>,
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
            emojis: emoji::create_discord_emoji_map(),
            connected: false,
            hook_enabled: false,
            chat_enabled: false,
            keyboard: Keyboard::new(),
            rx,
            tx,
        }
    }
}

impl eframe::App for Nagger {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.draw_ui(ctx, _frame);
    }
    
    fn raw_input_hook(&mut self, _ctx: &egui::Context, _raw_input: &mut egui::RawInput) {
        let events = self.keyboard.hook.poll();
        let mut result = Vec::new();

        for event in events {
            if let egui::Event::Key { key, pressed, modifiers, .. } = event {
                if pressed {
                    match key {
                        egui::Key::Slash => enable_hook(self, _raw_input),
                        egui::Key::Enter => self.hook_enabled = false,
                        egui::Key::Escape => self.hook_enabled = false,
                        _ => {},
                    };

                    if let Some(key_str) = key_to_str(&key, modifiers.shift) {
                        let event = egui::Event::Text(key_str.to_string());

                        result.push(event);
                    }
                } else {
                    match key {
                        egui::Key::Enter => disable_hook(self, _raw_input),
                        egui::Key::Escape => disable_hook(self, _raw_input),
                        _ => {},
                    }
                }
            }

            result.push(event);
        }

        _raw_input.focused = self.hook_enabled;
        _raw_input.events.append(&mut result);
    }
}

fn enable_hook(nagger: &mut Nagger, _raw_input: &mut egui::RawInput) {
    nagger.hook_enabled = true;
    #[cfg(target_os = "windows")]
    <keyboard::windows_hook::WindowsKeyboardHook as keyboard::KeyboardHook>::hook();
    #[cfg(target_os = "linux")]
    <keyboard::linux_hook::LinuxKeyboardHook as keyboard::KeyboardHook>::hook();
    _raw_input.focused = true;
}

fn disable_hook(nagger: &mut Nagger, _raw_input: &mut egui::RawInput) {
    nagger.hook_enabled = false;
    #[cfg(target_os = "windows")]
    <keyboard::windows_hook::WindowsKeyboardHook as keyboard::KeyboardHook>::unhook();
    #[cfg(target_os = "linux")]
    <keyboard::linux_hook::LinuxKeyboardHook as keyboard::KeyboardHook>::unhook();
    _raw_input.focused = false;
}

fn key_to_str(key: &Key, shift: bool) -> Option<&str> {
    match key {
        Key::Space => Some(" "),

        Key::Comma => if shift { Some("<") } else { Some(",") },
        Key::Period => if shift { Some(">") } else { Some(".") },
        Key::Backtick => if shift { Some("~") } else { Some("`") },
        Key::OpenBracket => if shift { Some("{") } else { Some("[") },
        Key::CloseBracket => if shift { Some("}") } else { Some("]") },
        Key::Backslash => if shift { Some("|") } else { Some("\\") },
        Key::Semicolon => if shift { Some(":") } else { Some(";") },
        Key::Quote => if shift { Some("\"") } else { Some("'") },

        Key::Num0 => if shift { Some(")") } else { Some("0") },
        Key::Num1 => if shift { Some("!") } else { Some("1") },
        Key::Num2 => if shift { Some("@") } else { Some("2") },
        Key::Num3 => if shift { Some("#") } else { Some("3") },
        Key::Num4 => if shift { Some("$") } else { Some("4") },
        Key::Num5 => if shift { Some("%") } else { Some("5") },
        Key::Num6 => if shift { Some("^") } else { Some("6") },
        Key::Num7 => if shift { Some("&") } else { Some("7") },
        Key::Num8 => if shift { Some("*") } else { Some("8") },
        Key::Num9 => if shift { Some("(") } else { Some("9") },
        Key::Minus => if shift { Some("_") } else { Some("-") },
        Key::Equals => if shift { Some("+") } else { Some("=") },

        Key::A => if shift { Some("A") } else { Some("a") },
        Key::B => if shift { Some("B") } else { Some("b") },
        Key::C => if shift { Some("C") } else { Some("c") },
        Key::D => if shift { Some("D") } else { Some("d") },
        Key::E => if shift { Some("E") } else { Some("e") },
        Key::F => if shift { Some("F") } else { Some("f") },
        Key::G => if shift { Some("G") } else { Some("g") },
        Key::H => if shift { Some("H") } else { Some("h") },
        Key::I => if shift { Some("I") } else { Some("i") },
        Key::J => if shift { Some("J") } else { Some("j") },
        Key::K => if shift { Some("K") } else { Some("k") },
        Key::L => if shift { Some("L") } else { Some("l") },
        Key::M => if shift { Some("M") } else { Some("m") },
        Key::N => if shift { Some("N") } else { Some("n") },
        Key::O => if shift { Some("O") } else { Some("o") },
        Key::P => if shift { Some("P") } else { Some("p") },
        Key::Q => if shift { Some("Q") } else { Some("q") },
        Key::R => if shift { Some("R") } else { Some("r") },
        Key::S => if shift { Some("S") } else { Some("s") },
        Key::T => if shift { Some("T") } else { Some("t") },
        Key::U => if shift { Some("U") } else { Some("u") },
        Key::V => if shift { Some("V") } else { Some("v") },
        Key::W => if shift { Some("W") } else { Some("w") },
        Key::X => if shift { Some("X") } else { Some("x") },
        Key::Y => if shift { Some("Y") } else { Some("y") },
        Key::Z => if shift { Some("Z") } else { Some("z") },
        _ => None,
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
    let viewport = egui::ViewportBuilder {
        icon: Some(Arc::new(icon_data)),
        transparent: Some(true),
        window_level: Some(egui::WindowLevel::AlwaysOnTop),
        // mouse_passthrough: Some(true),
        ..Default::default()
    };

    let native_options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        "Nagger", 
        native_options, 
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(Nagger::new(cc, events.clone(), ev_rx, cmd_tx, msg_rx)))
        })
    ).unwrap();
}