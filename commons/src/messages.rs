//! Module providing the websocket messages used in the application.
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Text(String),
    Ping,
    Pong,
}