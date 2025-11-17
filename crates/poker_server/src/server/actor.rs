use crate::server::safe_deserialise;
use axum::extract::ws::{Message, Utf8Bytes, WebSocket};
use log::error;
use poker::poker::{
    betting_strategy::BetArgs,
    card::{Card, Hand},
    compare::best_hand,
    game::{Bet, Stage},
    player::{Actor, Msg, Winner},
};
use serde::{Deserialize, Serialize};
use std::sync::mpsc as std_mpsc;
use tokio::{
    runtime::Handle,
    sync::{mpsc, oneshot},
};

// --- CONSTANTS ---
// We use mpsc for Server Updates -> WebSocket
const CHANNEL_CAPACITY: usize = 32;
// This type bundles the synchronous input data with the asynchronous reply channel.
type BetRequest = (BetArgs, (Card, Card), usize, oneshot::Sender<Option<Bet>>);

/// Enum for messages within a game.
#[derive(Debug, Serialize, Deserialize)]
pub enum PokerMessage {
    // Client -> Server messages
    PlayerBet(Bet),

    // Server -> Client messages
    Player {
        name: String,
        bank_roll: usize,
    },
    HoleCards {
        cards: (Card, Card),
    },
    BetPlaced {
        player: String,
        bet: Bet,
        pot: usize,
    },
    PlayersInfo {
        players: Vec<(String, usize)>,
        dealer: String,
    },
    GameWinner {
        winner: Winner,
    },
    RoundWinner {
        winner: Winner,
    },
    StageDecl {
        stage: Stage,
        community_cards: Vec<Card>,
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
    // Channel sender for sending updates (Msg) to the WebSocket loop.
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
    // Run the loop until the socket closes or an error occurs.
    loop {
        tokio::select! {
            // Handle incoming messages from the synchronous game loop (Updates and Bet Requests)
            // This is how the game engine tells the player to do something.
            Some((args, hole_cards, bank_roll, bet_responder)) = bet_rx.recv() => {
                // When the game engine calls place_bet, it sends a oneshot channel here.

                // Construct the bet request message.
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

                // Send the request to the client.
                let send_res = socket
                    .send(Message::Text(Utf8Bytes::from(
                        serde_json::to_string(&bet_msg).unwrap(),
                    )))
                    .await;

                if send_res.is_err() {
                    let _ = bet_responder.send(None);
                    return; // Exit loop on send error.
                }

                // Wait for the client's response (this is the actual blocking network IO).
                if let Some(msg) = socket.recv().await {
                    match msg {
                        Ok(Message::Text(utf8_bytes)) => {
                println!("Received bytes: {:?}", utf8_bytes);
                let bet = safe_deserialise::<PokerMessage>(&utf8_bytes);
                println!("Deserialised as: {:?}", bet);
                            // Extract the bet action.
                            let final_bet = match bet {
                                Some(PokerMessage::PlayerBet(b)) => Some(b),
                                _ => {
                                    error!("Expected PlayerBet, got something else: {:?}", bet);
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

            // Handle incoming updates from the synchronous game loop.
            Some(msg) = update_rx.recv() => {
                // General updates are sent to the client (fire-and-forget).
                let send_res = socket
                    .send(Message::Text(Utf8Bytes::from(
                        serde_json::to_string(&msg).unwrap(),
                    )))
                    .await;

                if send_res.is_err() {
                    return; // Exit loop on send error.
                }
            }

            // Handle messages spontaneously sent from the client.
            Some(result) = socket.recv() => {
                match result {
                    Ok(Message::Text(_utf8_bytes)) => {
                        // Handle unsolicited messages here (e.g., chat or keep-alive).
                        // Do nothing, since we only care about bet responses during place_bet.
                    }
                    Ok(Message::Close(_)) => {
                        error!("WebSocket closed by client.");
                        return;
                    }
                    Err(e) => {
                        error!("WebSocket error: {}", e);
                        return;
                    }
                    _ => {} // Ignore Binary, Ping, Pong, etc.
                }
            }

            // Exit if all senders are dropped.
            else => {
                break;
            }
        }
    }
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
        // Create channels for communication between the facade and the async loop.
        let (update_tx, update_rx) = mpsc::channel(CHANNEL_CAPACITY);
        let (bet_tx, bet_rx) = mpsc::channel(1); // Only need capacity 1 for blocking bets

        // Start the continuous asynchronous task that owns the WebSocket.
        runtime_handle.spawn(start_socket_loop(socket, update_rx, bet_rx));

        RemoteActor {
            runtime_handle,
            handle: RemoteActorHandle { update_tx, bet_tx },
        }
    }
}
/// Implementation of Actor for RemoteActor.
impl Actor for RemoteActor {
    /// Accept the name and bank roll at the beginning of the game.
    fn set_name_and_bank_roll(&self, name: &str, bank_roll: usize) {
        let msg = Msg::Player {
            name: name.to_string(),
            bank_roll,
        };
        self.update(&msg);
    }

    /// Accept the hole cards.
    fn hole_cards(&self, hole_cards: (Card, Card)) {
        let hole_card_msg = Msg::HoleCards { cards: hole_cards };
        self.update(&hole_card_msg);
    }
    /// Place a bet (Synchronous, Blocking).
    fn place_bet(
        &mut self,
        args: BetArgs,
        hole_cards: (Card, Card),
        bank_roll: usize,
    ) -> Option<Bet> {
        println!("Sending bet request: {:?}", args.clone());
        // Blocking MPSC channel for the final result.
        let (sync_tx, sync_rx) = std_mpsc::channel();

        // Clone necessary parts to move into the blocking thread
        let tx_clone = self.handle.bet_tx.clone();
        let runtime_handle_clone = self.runtime_handle.clone();

        // Spawn the entire request/response sequence onto a dedicated blocking thread.
        runtime_handle_clone.spawn_blocking(move || {
            // Tokio oneshot channel for receiving from the socket loop
            let (result_tx, result_rx) = oneshot::channel();
            let request: BetRequest = (args, hole_cards, bank_roll, result_tx);

            // Send the request to the async loop.
            if tx_clone.blocking_send(request).is_err() {
                error!("Failed to send bet request to socket loop from blocking thread.");
                let _ = sync_tx.send(None); // Send None back to the caller
                return;
            }

            // Block the current dedicated thread until the async loop replies.
            let result = match result_rx.blocking_recv() {
                Ok(bet) => bet,
                Err(e) => {
                    error!("Oneshot receiver error on blocking thread: {}", e);
                    None
                }
            };

            // Send the final result back to the original calling thread via std::mpsc.
            let _ = sync_tx.send(result);
        });

        // The thread calling `place_bet` blocks on the standard library receiver.
        match sync_rx.recv() {
            Ok(bet) => bet, // The Option<Bet> result
            Err(e) => {
                error!("Standard MPSC channel error waiting for result: {}", e);
                None
            }
        }
    }

    /// Update (Synchronous, Non-Blocking).
    fn update(&self, msg: &Msg) {
        println!("Sending update: {}", msg);
        // Convert (synchronous) Msg into (asynchronous) PokerMessage.
        let poker_msg = match msg {
            Msg::Player { name, bank_roll } => PokerMessage::Player {
                name: name.clone(),
                bank_roll: *bank_roll,
            },
            Msg::HoleCards { cards } => PokerMessage::HoleCards { cards: *cards },
            Msg::Bet { player, bet, pot } => PokerMessage::BetPlaced {
                player: player.clone(),
                bet: *bet,
                pot: *pot,
            },
            Msg::PlayersInfo { players, dealer } => PokerMessage::PlayersInfo {
                players: players.clone(),
                dealer: dealer.clone(),
            },
            Msg::GameWinner(winner) => PokerMessage::GameWinner {
                winner: winner.clone(),
            },
            Msg::RoundWinner(winner) => PokerMessage::RoundWinner {
                winner: winner.clone(),
            },
            Msg::StageDeclare(stage, community_cards) => PokerMessage::StageDecl {
                stage: *stage,
                community_cards: community_cards.clone(),
            },
        };
        let tx = self.handle.update_tx.clone();
        self.runtime_handle.spawn(async move {
            if let Err(e) = tx.send(poker_msg).await {
                error!("Failed to send update message: {}", e);
            }
        });
    }
}
