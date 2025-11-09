mod server;
use crate::server::game::game_handler;
use axum::{
    Router,
    extract::{
        ConnectInfo,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::IntoResponse,
    routing::get,
};
use log::error;
use log::info;
use serde::{Deserialize, Serialize};
use server::{
    config::Settings,
    {safe_deserialise, send_close_message},
};
use std::{env, net::SocketAddr};

/// Enum for join game messages only.
#[derive(Debug, Serialize, Deserialize)]
pub enum GameRequest {
    NewGame { name: String },
    JoinGame { game_id: String, username: String },
}

/// Extractor for establishing WebSocket connections.
async fn websocket_handler(
    ws: WebSocketUpgrade,
    ConnectInfo(remote_addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, remote_addr))
}

/// A stream of WebSocket messages.
async fn handle_socket(mut socket: WebSocket, remote_addr: SocketAddr) {
    // Returns `None` if the stream has closed.
    if let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            // We only consider text messages. Ignore binary, ping, pong.
            if let Message::Text(utf8_bytes) = msg
                && let Some(game_request) = safe_deserialise(&utf8_bytes)
            {
                info!("Game request from {}", remote_addr);
                let runtime_handle = tokio::runtime::Handle::current();
                match game_request {
                    GameRequest::NewGame { name } => {
                        game_handler(name, socket, runtime_handle).await;
                    }
                    GameRequest::JoinGame { .. } => {}
                }
            }
        } else {
            let error = msg.err().unwrap();
            error!("Error receiving message: {:?}", error);
            send_close_message(socket, 1011, &format!("Error occured: {}", error)).await;
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    // Load config.
    Ok(match Settings::load(args) {
        Ok(settings) => {
            let app = Router::new().route("/", get(websocket_handler));
            let address = settings.server.host + ":" + &settings.server.port.to_string();
            info!("Starting server at address: {}", address);
            let listener = tokio::net::TcpListener::bind(address).await.unwrap();

            axum::serve(
                listener,
                app.into_make_service_with_connect_info::<SocketAddr>(),
            )
            .await
            .unwrap();

            anyhow::Result::Ok(())
        }
        Err(e) => {
            error!("Error loading config: {}", e);
            anyhow::Result::Err(e)
        }
    }?)
}
