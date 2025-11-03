use axum::Router;
use axum::extract::ws::{CloseFrame, Utf8Bytes};
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
};
use poker::poker::card::Hand;
use poker::poker::game::{Bet, Stage};
use serde::{Deserialize, Serialize};

/// Struct for duplex communication.
#[derive(Debug, Serialize, Deserialize)]
pub enum PokerMessage {
    // Client -> Server messages
    NewGame { name: String },
    JoinGame { game_id: String, username: String },
    PlayerAction { action_type: String, amount: usize },

    // Server -> Client messages
    StageUpdate { stage: Stage },
    PlayerUpdate { player: String, bet: Bet },
    RoundUpdate { winner: String, hand: Hand },
    Error(String),
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
    while let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            match msg {
                Message::Text(utf8_bytes) => {
                    println!("Text received: {}", utf8_bytes);
                    let dec = deserialise(&utf8_bytes);
                    println!("Received: {:?}", dec);
                    let msg = PokerMessage::PlayerUpdate {
                        player: "James".to_string(),
                        bet: Bet::Raise(200),
                    };
                    let result = socket
                        .send(Message::Text(Utf8Bytes::from(
                            serde_json::to_string(&msg).unwrap(),
                        )))
                        .await;
                    if let Err(error) = result {
                        println!("Error sending: {}", error);
                        send_close_message(socket, 1011, &format!("Error occured: {}", error))
                            .await;
                        break;
                    }
                }
                Message::Binary(bytes) => {
                    println!("Received bytes of length: {}", bytes.len());
                    let result = socket
                        .send(Message::Text(
                            format!("Received bytes of length: {}", bytes.len()).into(),
                        ))
                        .await;
                    if let Err(error) = result {
                        println!("Error sending: {}", error);
                        send_close_message(socket, 1011, &format!("Error occured: {}", error))
                            .await;
                        break;
                    }
                }
                _ => {}
            }
        } else {
            let error = msg.err().unwrap();
            println!("Error receiving message: {:?}", error);
            send_close_message(socket, 1011, &format!("Error occured: {}", error)).await;
            break;
        }
    }
}

// Graceful closing protocol.
async fn send_close_message(mut socket: WebSocket, code: u16, reason: &str) {
    _ = socket
        .send(Message::Close(Some(CloseFrame {
            code,
            reason: reason.into(),
        })))
        .await;
}

fn deserialise(utf8_bytes: &Utf8Bytes) -> PokerMessage {
    let msg = str::from_utf8(utf8_bytes.as_bytes()).unwrap();
    let o = serde_json::from_str(msg).unwrap();
    o
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new().route("/", get(websocket_handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    axum::serve(listener, app).await?;

    Ok(())
}
