use crate::server::deserialise_pokermessage;
use axum::extract::ws::{Message, Utf8Bytes, WebSocket};
use poker::poker::{
    betting_strategy::BetArgs,
    card::{Card, Hand},
    compare::best_hand,
    game::Bet,
    player::{Actor, Msg},
};
use serde::{Deserialize, Serialize};
use tokio::{
    runtime::Handle,
    sync::{mpsc, oneshot},
};

// --- CONSTANTS ---
// We use mpsc for Server Updates -> WebSocket
const CHANNEL_CAPACITY: usize = 32;
// --- TYPE ALIAS FOR BET REQUEST (NEW) ---
// This bundles the synchronous input data with the asynchronous reply channel.
type BetRequest = (BetArgs, (Card, Card), usize, oneshot::Sender<Option<Bet>>);

/// Enum for messages within a game.
#[derive(Debug, Serialize, Deserialize)]
pub enum PokerMessage {
    // Client -> Server messages
    PlayerAction {
        action_type: String,
        amount: usize,
    },
    PlayerBet(Bet),

    // Server -> Client messages
    General {
        msg: Msg,
    },
    PlaceBet {
        args: BetArgs,
        hole_cards: (Card, Card),
        bank_roll: usize,
        best_hand: Hand,
    },
    Error(String),
}
/// The thread-safe, cloneable structure used by the synchronous facade
/// to push messages into the asynchronous WebSocket task.
#[derive(Debug, Clone)]
struct RemoteActorHandle {
    // Channel sender for sending updates (Msg) to the WebSocket loop
    update_tx: mpsc::Sender<PokerMessage>,
    // Channel sender for requesting a bet (place_bet) and receiving the result
    // The player thread sends a request to the loop, and the loop replies here.
    bet_tx: mpsc::Sender<BetRequest>,
}
/// The actual asynchronous loop that manages the single, non-cloneable WebSocket.
async fn start_socket_loop(
    mut socket: WebSocket,
    mut update_rx: mpsc::Receiver<PokerMessage>,
    mut bet_rx: mpsc::Receiver<BetRequest>,
) {
    // Run the loop indefinitely until the socket closes or an error occurs
    loop {
        tokio::select! {
            // 1. Handle incoming messages from the synchronous game loop (Updates and Bet Requests)
            // This is how the game engine tells the player to do something.
            Some((args, hole_cards, bank_roll, bet_responder)) = bet_rx.recv() => {
                // When the game engine calls place_bet, it sends a oneshot channel here.

                // Construct the bet request message
                let mut cards = args.community_cards.clone();
        let (h1, h2) = (hole_cards.0, hole_cards.1);
        cards.push(h1);
        cards.push(h2);
        let bh = best_hand(&cards);
                let bet_msg = PokerMessage::PlaceBet {
                    args,
                    hole_cards,
                    bank_roll,
                    best_hand: bh,
                };

                // Send the request to the client
                let send_res = socket
                    .send(Message::Text(Utf8Bytes::from(
                        serde_json::to_string(&bet_msg).unwrap(),
                    )))
                    .await;

                if send_res.is_err() {
                    let _ = bet_responder.send(None);
                    return; // Exit loop on send error
                }

                // Wait for the client's response (this is the actual blocking network IO)
                if let Some(msg) = socket.recv().await {
                    match msg {
                        Ok(Message::Text(utf8_bytes)) => {
                            // In a real app, deserialise_pokermessage would parse this
                            let bet = deserialise_pokermessage(&utf8_bytes);

                            // Extract the bet action
                            let final_bet = match bet {
                                PokerMessage::PlayerBet(b) => Some(b),
                                _ => {
                                    eprintln!("Expected PlayerBet, got something else: {:?}", bet);
                                    None
                                }
                            };
                            let _ = bet_responder.send(final_bet);
                        }
                        _ => {
                            eprintln!("Received non-text message or error during recv.");
                            let _ = bet_responder.send(None);
                        }
                    }
                } else {
                    // Recv returned None, meaning socket closed.
                    let _ = bet_responder.send(None);
                    return;
                }
            }

            // 2. Handle incoming updates from the synchronous game loop
            Some(msg) = update_rx.recv() => {
                // General updates are sent to the client (fire-and-forget)
                let send_res = socket
                    .send(Message::Text(Utf8Bytes::from(
                        serde_json::to_string(&msg).unwrap(),
                    )))
                    .await;

                if send_res.is_err() {
                    return; // Exit loop on send error
                }
            }

            // 3. Handle messages spontaneously sent from the client (e.g., chat)
            Some(result) = socket.recv() => {
                match result {
                    Ok(Message::Text(utf8_bytes)) => {
                        // Handle unsolicited messages here (e.g., chat or keep-alive)
                        let _msg = deserialise_pokermessage(&utf8_bytes);
                        // Do nothing, since we only care about bet responses during place_bet
                    }
                    Ok(Message::Close(_)) => {
                        println!("WebSocket closed by client.");
                        return;
                    }
                    Err(e) => {
                        eprintln!("WebSocket error: {}", e);
                        return;
                    }
                    _ => {} // Ignore Binary, Ping, Pong, etc.
                }
            }

            // Exit if all senders are dropped (no more messages to manage)
            else => {
                break;
            }
        }
    }
    // Clean up or log closure here
    println!("WebSocket loop finished.");
}

/// The Synchronous Facade struct that implements the Actor trait.
#[derive(Debug)]
pub struct RemoteActor {
    runtime_handle: Handle,
    handle: RemoteActorHandle,
}
impl RemoteActor {
    /// Builds a new RemoteActor, starts the asynchronous WebSocket loop, and returns the facade.
    pub fn build(socket: WebSocket, runtime_handle: Handle) -> RemoteActor {
        // Create channels for communication between the facade and the async loop
        let (update_tx, update_rx) = mpsc::channel(CHANNEL_CAPACITY);
        let (bet_tx, bet_rx) = mpsc::channel(1); // Only need capacity 1 for blocking bets

        // Start the continuous asynchronous task that owns the WebSocket
        runtime_handle.spawn(start_socket_loop(socket, update_rx, bet_rx));

        RemoteActor {
            runtime_handle,
            handle: RemoteActorHandle { update_tx, bet_tx },
        }
    }
}
// NOTE: The Actor trait methods must be defined here, NOT in the provided code snippet
// I have created a simplified, conceptual impl for demonstration purposes.
impl Actor for RemoteActor {
    /// Place a bet (Synchronous, Blocking).
    fn place_bet(
        &mut self, // Changed to &self as state is shared via channels
        args: BetArgs,
        hole_cards: (Card, Card),
        bank_roll: usize,
    ) -> Option<Bet> {
        // 1. Create a oneshot channel to receive the result from the async loop
        let (result_tx, result_rx) = oneshot::channel();
        let request: BetRequest = (args, hole_cards, bank_roll, result_tx);
        // 2. Send the receiver channel to the async loop
        let send_result = self.handle.bet_tx.blocking_send(request);

        if send_result.is_err() {
            eprintln!("Failed to send bet request to socket loop.");
            return None;
        }

        // 3. Block on the result channel, waiting for the async loop to reply
        match self.runtime_handle.block_on(result_rx) {
            Ok(bet) => bet, // This is the Option<Bet> sent by the async loop
            Err(e) => {
                eprintln!("WebSocket loop failed to respond to bet request: {}", e);
                None
            }
        }
    }

    /// Update (Synchronous, Non-Blocking).
    fn update(&self, msg: &Msg) {
        // Convert the synchronous Msg into the asynchronous PokerMessage
        let poker_msg = PokerMessage::General { msg: msg.clone() };

        // Use blocking_send to avoid spawning another future just for the send.
        // This is safe because mpsc::Sender::blocking_send is non-blocking
        // until the channel capacity is reached (which is why we set capacity=32).
        if let Err(e) = self.handle.update_tx.blocking_send(poker_msg) {
            eprintln!("Failed to send update message: {}", e);
        }
    }
}
/*
/// Using the Synchronous Facade pattern to hide the asynchronicity of remote
/// actors, enabling them to implement the (synchronous) Actor Trait.
/// The async part is behind a level of indirection in RemoteActorImpl.
#[derive(Debug)]
pub struct RemoteActorImpl {
    // e.g., a connection to the client
    socket: WebSocket,
}
/// Implementation for RemoteActorImpl
impl RemoteActorImpl {
    async fn remote_place_bet(
        &mut self,
        args: BetArgs,
        hole_cards: (Card, Card),
        bank_roll: usize,
    ) -> Option<Bet> {
        let mut cards = args.community_cards.clone();
        let (h1, h2) = (hole_cards.0, hole_cards.1);
        cards.push(h1);
        cards.push(h2);
        let bh = best_hand(&cards);

        let msg = PokerMessage::PlaceBet {
            args,
            hole_cards,
            bank_roll,
            best_hand: bh,
        };
        let result = self
            .socket
            .send(Message::Text(Utf8Bytes::from(
                serde_json::to_string(&msg).unwrap(),
            )))
            .await;
        // then get the result
        if let Some(msg) = self.socket.recv().await {
            if let Ok(msg) = msg {
                match msg {
                    Message::Text(utf8_bytes) => {
                        let bet = deserialise_pokermessage(&utf8_bytes);
                        match bet {
                            PokerMessage::PlayerBet(bet) => match bet {
                                Bet::Fold => Some(Bet::Fold),
                                Bet::Check => Some(Bet::Check),
                                Bet::Call => Some(Bet::Call),
                                Bet::Raise(n) => Some(Bet::Raise(n)),
                                Bet::AllIn(n) => Some(Bet::AllIn(n)),
                            },
                            _ => {
                                dbg!("Was expecting a bet but got {}",);
                                None
                            }
                        }
                    }
                    _ => None,
                }
            } else {
                None
            }
        } else if let Err(error) = result {
            println!("Error sending: {}", error);
            crate::server::send_close_message(
                self.socket,
                1011,
                &format!("Error occured: {}", error),
            )
            .await;
            None
        }
    }

    async fn remote_update(&mut self, msg: &Msg) {
        self.socket
            .send(Message::Text(Utf8Bytes::from(
                serde_json::to_string(msg).unwrap(),
            )))
            .await;
    }

    // async fn remote_update(&self, msg: Msg) { /* ... */ }
}
#[derive(Debug)]
pub struct RemoteActor {
    runtime_handle: Handle,
    remote_impl: RemoteActorImpl,
}

impl RemoteActor {
    pub fn build(socket: WebSocket, runtime_handle: Handle) -> RemoteActor {
        RemoteActor {
            runtime_handle,
            remote_impl: RemoteActorImpl { socket },
        }
    }
}

impl Actor for RemoteActor {
    /// Place a bet.
    fn place_bet(
        &mut self,
        args: BetArgs,
        hole_cards: (Card, Card),
        bank_roll: usize,
    ) -> Option<Bet> {
        // Use the runtime handle to block on the async call
        let future = self
            .remote_impl
            .remote_place_bet(args, hole_cards, bank_roll);
        self.runtime_handle.block_on(future)
    }

    fn update(&self, msg: &Msg) {
        let mut remote_impl = &self.remote_impl;
        let msg = msg.clone();
        self.runtime_handle.spawn(async move {
            remote_impl.remote_update(&msg).await;
        });
    }
}
*/
