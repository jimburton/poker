/// Datatypes and functions for players in the game.
use crate::poker::betting_strategy;
use crate::poker::betting_strategy::{BetArgs, BettingStrategy};
use crate::poker::card::{Card, Hand};
use crate::poker::game::{Bet, Stage};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug, Display};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerHand {
    pub name: String,
    pub hand: Hand,
    pub cards: Vec<Card>,
}
/// Messages to send to players.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Msg {
    Bet { player: String, bet: Bet },
    Misc(String),
    Game(Winner),
    Round(Stage),
}
/// Implementation of Display trait for Msg.
impl Display for Msg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Msg::Bet { player, bet } => write!(f, "{} made bet {}", player, bet),
            Msg::Misc(msg) => write!(f, "{}", msg),
            Msg::Game(winner) => write!(f, "{}", winner),
            Msg::Round(stage) => write!(f, "{}", stage),
        }
    }
}

/// Enum for representing the winner(s) of a round.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Winner {
    Winner(PlayerHand),
    Draw(Vec<PlayerHand>),
}
/// Implementation of Display trait for Winner.
impl Display for Winner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Winner::Winner(PlayerHand { name, hand, .. }) => {
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
    /// Place a bet by asking the actor to do it.
    pub fn place_bet(&mut self, args: BetArgs) -> Option<Bet> {
        let bet = self
            .actor
            .place_bet(args.clone(), self.hole.unwrap(), self.bank_roll)
            .unwrap();
        match bet {
            Bet::Fold => {
                self.folded = true;
                Some(Bet::Fold)
            }
            Bet::Check => Some(Bet::Check),
            Bet::Call => {
                self.bank_roll -= args.clone().call;
                Some(Bet::Call)
            }
            Bet::Raise(n) => {
                self.bank_roll -= n;
                Some(Bet::Raise(n))
            }
            Bet::AllIn(n) => {
                self.bank_roll = 0;
                Some(Bet::AllIn(n))
            }
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

/// The actor for a computer player.
#[derive(Debug, Clone)]
pub struct AutoActor {
    pub betting_strategy: BettingStrategy,
}

/// Implementation for AutoActor.
impl AutoActor {
    /// Construct a new Player instance.
    pub fn new() -> Self {
        AutoActor {
            betting_strategy: betting_strategy::default_betting_strategy,
        }
    }
    /// Construct a new Player instance with the supplied strategy.
    pub fn build(betting_strategy: BettingStrategy) -> Self {
        AutoActor { betting_strategy }
    }
}
/// Implementation of Default trait for AutoActor.
impl Default for AutoActor {
    fn default() -> Self {
        Self::new()
    }
}
/// Implementation of the Actor trait for AutoActor.
impl Actor for AutoActor {
    /// Place a bet using the betting strategy.
    fn place_bet(
        &mut self,
        args: BetArgs,
        hole_cards: (Card, Card),
        bank_roll: usize,
    ) -> Option<Bet> {
        //let mut cards = args.community_cards.clone();
        //let (h1, h2) = hole_cards;
        //cards.push(h1);
        //cards.push(h2);
        let strategy = self.betting_strategy;
        Some(strategy(args, hole_cards, bank_roll))
    }

    /// Accept a message and do nothing with it.
    fn update(&self, _msg: &Msg) {}
}

#[cfg(test)]
mod tests {
    use super::*;

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
