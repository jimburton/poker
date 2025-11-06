use actor::PokerMessage;
use axum::extract::ws::{CloseFrame, Message, Utf8Bytes, WebSocket};

use crate::GameRequest;

pub mod actor;
pub mod game;

/// Graceful closing protocol.
pub async fn send_close_message(mut socket: WebSocket, code: u16, reason: &str) {
    _ = socket
        .send(Message::Close(Some(CloseFrame {
            code,
            reason: reason.into(),
        })))
        .await;
}

/// Deserialise a GameRequest struct.
pub fn deserialise_gamerequest(utf8_bytes: &Utf8Bytes) -> GameRequest {
    let msg = str::from_utf8(utf8_bytes.as_bytes()).unwrap();
    let o = serde_json::from_str(msg).unwrap();
    o
}

/// Deserialise a PokerMessage struct.
pub fn deserialise_pokermessage(utf8_bytes: &Utf8Bytes) -> PokerMessage {
    let msg = str::from_utf8(utf8_bytes.as_bytes()).unwrap();
    let o = serde_json::from_str(msg).unwrap();
    o
}
