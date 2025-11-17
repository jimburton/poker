/// Datatypes and functions for players in the game.
use crate::poker::{
    betting_strategy::BetArgs,
    card::{BestHand, Card},
    game::{Bet, Stage},
};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug, Display};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerHand {
    pub name: String,
    pub hand: BestHand,
    pub cards: Vec<Card>,
}
/// Messages to send to players.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Msg {
    Player {
        name: String,
        bank_roll: usize,
    },
    HoleCards {
        cards: (Card, Card),
    },
    Bet {
        player: String,
        bet: Bet,
        pot: usize,
    },
    PlayersInfo {
        players: Vec<(String, usize)>,
        dealer: String,
    }, // (name, bank roll)
    GameWinner(Winner),
    RoundWinner(Winner),
    StageDeclare(Stage, Vec<Card>),
}
/// Implementation of Display trait for Msg.
impl Display for Msg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Msg::Player { name, bank_roll } => write!(f, "Playing as {} ({})", name, bank_roll),
            Msg::HoleCards { cards } => write!(f, "Received hole cards {}, {}", cards.0, cards.1),
            Msg::Bet { player, bet, pot } => {
                write!(f, "{} made bet {} (pot is now {})", player, bet, pot)
            }
            Msg::PlayersInfo { players, dealer } => {
                write!(
                    f,
                    "Playing with {}",
                    players
                        .iter()
                        .map(|(player_name, bank_roll)| {
                            let mut name = player_name.clone();
                            if player_name == dealer {
                                name += " [Dealer]";
                            }
                            name + " (" + &bank_roll.to_string() + ")"
                        })
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            Msg::GameWinner(winner) => write!(f, "Won the game: {}", winner),
            Msg::RoundWinner(winner) => write!(f, "Won the round: {}", winner),
            Msg::StageDeclare(stage, community_cards) => {
                let cards_str = community_cards
                    .iter()
                    .map(|c| std::format!("{}", c))
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "{}, community cards: {}", stage, cards_str)
            }
        }
    }
}

/// Enum for representing the winner(s) of a round.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Winner {
    SoleWinner(PlayerHand),
    Draw(Vec<PlayerHand>),
}
/// Implementation of Display trait for Winner.
impl Display for Winner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Winner::SoleWinner(PlayerHand { name, hand, .. }) => {
                write!(f, "Winner: {} ({})", name, hand)
            }
            Winner::Draw(hands) => {
                let names = hands
                    .iter()
                    .map(|phand| format!("{} ({})", phand.name, phand.hand))
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "Draw: {}", names)
            }
        }
    }
}
/// The Actor trait is the component part of players which places bets
/// and responds to messages.
pub trait Actor: Debug {
    /// Accept the name and bank roll at the beginning of the game.
    fn set_name_and_bank_roll(&self, name: &str, bank_roll: usize) -> ();

    /// Accept the hole cards at the beginning of a round.
    fn hole_cards(&self, hole_cards: (Card, Card)) -> ();

    /// Place a bet.
    fn place_bet(
        &mut self,
        args: BetArgs,
        hole_cards: (Card, Card),
        bank_roll: usize,
    ) -> Option<Bet>;

    /// Receive an update message, e.g. the status of the game or information about the
    /// winner of a round or game.
    fn update(&self, msg: &Msg) -> ();
}
/// The Player struct.
#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub hole: Option<(Card, Card)>,
    pub bet: usize,
    pub bank_roll: usize,
    pub all_in: bool,
    pub folded: bool,
    pub actor: Box<dyn Actor + 'static>,
}
/// Implementation of Player.
impl Player {
    /// Construct a Player instance with the supplied actor.
    pub fn build(name: &str, actor: impl Actor + 'static) -> Player {
        Player {
            name: name.to_string(),
            hole: None,
            bet: 0,
            bank_roll: 0,
            all_in: false,
            folded: false,
            actor: Box::new(actor),
        }
    }

    /// Set name and bank roll at the beginning of a game. Needed because
    /// the name might need to be changed to become unique, and so that
    /// this info can be passed to remote clients.
    pub fn set_name_and_bank_roll(&mut self, name: &str, bank_roll: usize) {
        self.name = name.to_owned();
        self.bank_roll = bank_roll;
        self.actor.set_name_and_bank_roll(name, bank_roll);
    }

    /// Accept the hole cards and pass them on to the actor.
    pub fn hole_cards(&mut self, (h1, h2): (Card, Card)) {
        self.hole = Some((h1, h2));
        self.actor.hole_cards((h1, h2));
    }

    /// Place a bet by asking the actor to do it.
    pub fn place_bet(&mut self, args: BetArgs) -> Option<Bet> {
        if !self.all_in && !self.folded {
            let bet_opt = self
                .actor
                .place_bet(args.clone(), self.hole.unwrap(), self.bank_roll);
            if let Some(bet) = bet_opt {
                match bet {
                    Bet::Fold => {
                        self.folded = true;
                        Some(Bet::Fold)
                    }
                    Bet::Check => Some(Bet::Check),
                    Bet::Call => {
                        self.bank_roll -= args.call;
                        Some(Bet::Call)
                    }
                    Bet::Raise(n) => {
                        self.bank_roll -= n;
                        Some(Bet::Raise(n))
                    }
                    Bet::AllIn(n) => {
                        self.bank_roll = 0;
                        self.all_in = true;
                        Some(Bet::AllIn(n))
                    }
                }
            } else {
                panic!("No bet received from player.");
            }
        } else {
            None
        }
    }

    /// Respond to an incoming message by asking the actor to do it.
    pub fn update(&self, msg: &Msg) {
        self.actor.update(msg);
    }

    /// Pay the required amount to join a round.
    pub fn ante_up(&mut self, blind: usize) -> Option<usize> {
        if self.bank_roll > blind {
            self.bank_roll -= blind;
            Some(blind)
        } else if self.bank_roll > 0 {
            self.all_in = true;
            let bank_roll = self.bank_roll;
            self.bank_roll = 0;
            Some(bank_roll)
        } else {
            self.folded = true;
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::poker::AutoActor;
    #[test]
    fn test_build() {
        let player = Player::build("James", AutoActor::new());
        assert!(
            player.name == "James",
            "Expected new player to have name=='James', was {}",
            player.name
        );
    }
}
