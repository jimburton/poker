mod server;
use crate::server::game::game_handler;
use axum::{
    Router,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
};
use serde::{Deserialize, Serialize};
use server::{deserialise_gamerequest, send_close_message};

/// Enum for join game messages only.
#[derive(Debug, Serialize, Deserialize)]
pub enum GameRequest {
    NewGame { name: String },
    JoinGame { game_id: String, username: String },
}

/// Extractor for establishing WebSocket connections.
async fn websocket_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    // Finalize upgrading the connection and call the provided callback with the stream.
    ws.on_failed_upgrade(|error| println!("Error upgrading websocket: {}", error))
        .on_upgrade(handle_socket)
}

/// A stream of WebSocket messages.
async fn handle_socket(mut socket: WebSocket) {
    // Returns `None` if the stream has closed.
    if let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            match msg {
                Message::Text(utf8_bytes) => {
                    println!("Text received: {}", utf8_bytes);
                    let dec = deserialise_gamerequest(&utf8_bytes);
                    println!("Received: {:?}", dec);
                    let runtime_handle = tokio::runtime::Handle::current();
                    match dec {
                        GameRequest::NewGame { name } => {
                            game_handler(name, socket, runtime_handle).await;
                        }
                        GameRequest::JoinGame { .. } => {}
                    }
                }
                _ => {} // ignore binary, ping, pong
            }
        } else {
            let error = msg.err().unwrap();
            println!("Error receiving message: {:?}", error);
            send_close_message(socket, 1011, &format!("Error occured: {}", error)).await;
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new().route("/", get(websocket_handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    axum::serve(listener, app).await?;

    Ok(())
}
