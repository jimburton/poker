use axum::extract::ws::{Message, Utf8Bytes, WebSocket};
use poker::poker::betting_strategy::BetArgs;
use poker::poker::card::{Card, Hand};
use poker::poker::compare::best_hand;
use poker::poker::game::Bet;
use poker::poker::player::{Actor, Msg};
use serde::{Deserialize, Serialize};

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
#[derive(Debug)]
pub struct RemoteActor {
    socket: WebSocket,
}

impl RemoteActor {
    pub fn build(socket: WebSocket) -> RemoteActor {
        RemoteActor { socket }
    }
}

impl Actor for RemoteActor {
    /// Place a bet.
    async fn place_bet(
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
        if let Some(msg) = self.socket.recv().await {
            if let Ok(msg) = msg {
                match msg {
                    Message::Text(utf8_bytes) => {
                        if let Some(bet) = deserialise(&utf8_bytes) {
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
                        } else {
                            dbg!("Was expecting a bet but got {:?}", msg);
                            None
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
        }
    }

    async fn update(&self, msg: &Msg) {
        let result = self
            .socket
            .send(Message::Text(Utf8Bytes::from(
                serde_json::to_string(&msg).unwrap(),
            )))
            .await;
    }
}

fn deserialise(utf8_bytes: &Utf8Bytes) -> Option<PokerMessage, &'static str> {
    let msg = str::from_utf8(utf8_bytes.as_bytes()).unwrap();
    let o = serde_json::from_str(msg).unwrap();
    o
}
