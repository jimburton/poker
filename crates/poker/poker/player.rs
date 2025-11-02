/// Datatypes and functions for players in the game.
use crate::poker::betting_strategy;
use crate::poker::card::{Card, Hand};
use crate::poker::game::{Bet, Stage};
use std::fmt::{self, Debug, Display};

use super::betting_strategy::{BettingStrategy, StrategyArgs};
/// A utility struct with information about a player's hand, with the
/// cards being made up of their hole cards and the available community cards.
#[derive(Debug, Clone)]
pub struct PlayerHand {
    pub name: String,
    pub best_hand: Hand,
    pub cards: Vec<Card>,
}

#[derive(Debug, Clone)]
pub enum Msg {
    Round(Stage),
    Bet { player: String, bet: Bet },
    Misc(String),
    Winner(Winner),
}
impl Display for Msg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Msg::Round(stage) => write!(f, "{}", stage),
            Msg::Bet { player, bet } => write!(f, "{} made bet {}", player, bet),
            Msg::Misc(msg) => write!(f, "{}", msg),
            Msg::Winner(winner) => write!(f, "Winner: {}", winner),
        }
    }
}

/// Enum for representing the winner(s) of a round.
#[derive(Debug, Clone)]
pub enum Winner {
    Winner {
        name: String,
        hand: Hand,
        cards: Vec<Card>,
    },
    Draw(Vec<PlayerHand>),
}
impl Display for Winner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Winner::Winner { name, hand, .. } => write!(f, "Winner: {} ({})", name, hand),
            Winner::Draw(hands) => {
                let names = hands
                    .iter()
                    .map(|phand| format!("{} ({})", phand.name, phand.best_hand))
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "Draw: {}", names)
            }
        }
    }
}

/// Struct for the winner of the game
#[derive(Debug)]
pub struct WinnerInfo {
    pub name: String,
    pub num_rounds: usize,
    pub winnings: usize,
}
impl Display for WinnerInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} won the game with {} winnings after {} rounds",
            self.name, self.winnings, self.num_rounds
        )
    }
}

/// Struct for the winner of a round
#[derive(Debug)]
pub struct RoundWinnerInfo {
    pub name: String,
    pub hand: Hand,
    pub winnings: usize,
}
impl Display for RoundWinnerInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} won {} with {}", self.name, self.winnings, self.hand)
    }
}

pub trait Actor: Debug {
    /// Place a bet.
    fn place_bet(
        &mut self,
        call: usize,
        min: usize,
        stage: Stage,
        cycle: u8,
        bank_roll: usize,
        community_cards: Vec<Card>,
        hole_cards: (Card, Card),
    ) -> Option<Bet>;

    /// Receive an update message, e.g. the status of the game or information about the
    /// winner of a round or game.
    fn update(&self, msg: &Msg) -> ();
}
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

impl Player {
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

    pub fn place_bet(
        &mut self,
        call: usize,
        min: usize,
        community_cards: Vec<Card>,
        stage: Stage,
        cycle: u8,
    ) -> Option<Bet> {
        let bet = self
            .actor
            .place_bet(
                call,
                min,
                stage,
                cycle,
                self.bank_roll,
                community_cards,
                self.hole.unwrap(),
            )
            .unwrap();
        match bet {
            Bet::Fold => {
                self.folded = true;
                Some(Bet::Fold)
            }
            Bet::Check => Some(Bet::Check),
            Bet::Call => {
                self.bank_roll -= call;
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

    pub fn update(&self, msg: &Msg) {
        self.actor.update(msg);
    }

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

/// The struct that represents a computer player.
#[derive(Debug, Clone)]
pub struct AutoActor {
    pub betting_strategy: BettingStrategy,
}

/// Implementation for the Player struct.
impl AutoActor {
    /// Construct a new Player.
    pub fn new() -> Self {
        AutoActor {
            betting_strategy: betting_strategy::default_betting_strategy,
        }
    }

    pub fn build(betting_strategy: BettingStrategy) -> Self {
        AutoActor { betting_strategy }
    }
}

impl Actor for AutoActor {
    /// Place a bet.
    fn place_bet(
        &mut self,
        call: usize,
        min: usize,
        stage: Stage,
        cycle: u8,
        bank_roll: usize,
        community_cards: Vec<Card>,
        hole_cards: (Card, Card),
    ) -> Option<Bet> {
        let mut cards = community_cards.clone();
        let (h1, h2) = hole_cards;
        cards.push(h1);
        cards.push(h2);
        let strategy = self.betting_strategy;
        let args = StrategyArgs {
            call,
            min,
            bank_roll,
            cards,
            stage,
            cycle,
        };
        Some(strategy(args))
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
