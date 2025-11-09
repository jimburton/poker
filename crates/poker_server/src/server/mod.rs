use axum::extract::ws::{CloseFrame, Message, WebSocket};
use log::error;
use serde::Deserialize;

pub mod actor;
pub mod config;
pub mod game;
mod log_setup;

/// Graceful closing protocol.
pub async fn send_close_message(mut socket: WebSocket, code: u16, reason: &str) {
    _ = socket
        .send(Message::Close(Some(CloseFrame {
            code,
            reason: reason.into(),
        })))
        .await;
}

/// Safely attempts to deserialize a UTF-8 string slice into any type T
/// that implements the Deserialize trait.
pub fn safe_deserialise<'a, T>(bytes: &'a str) -> Option<T>
where
    T: Deserialize<'a>,
{
    match serde_json::from_str(bytes) {
        Ok(data) => Some(data),
        Err(e) => {
            error!("Deserialization error: {}", e);
            None
        }
    }
}
