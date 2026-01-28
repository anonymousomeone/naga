use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub text: String,
    pub user: User
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub username: String,
    pub color: Color,
    pub system: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    pub event_type: EventType,
    pub time: u128
}

#[derive(Serialize, Deserialize, Debug)]
pub enum EventType {
    Message(Message),
    Joined(User),
    Left(User),
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}