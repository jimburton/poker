/// Datatypes and functions for players in the game.
use crate::poker::betting_strategy;
use crate::poker::card::{Card, Hand};
use crate::poker::game::{Bet, Stage};
use std::fmt::Debug;

use super::betting_strategy::BettingStrategy;
/// A utility struct with information about a player's hand, with the
/// cards being made up of their hole cards and the available community cards.
#[derive(Debug, Clone)]
pub struct PlayerHand {
    pub name: String,
    pub best_hand: Hand,
    pub cards: Vec<Card>,
}

#[derive(Debug, Clone)]
pub struct Update {
    pub player: String,
    pub bet: Bet,
}

#[derive(Debug, Clone)]
pub enum Msg {
    MsgBet(Update),
    MsgMisc(String),
    MsgWinner(Winner),
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

/// Struct for the winner of the game
#[derive(Debug)]
pub struct WinnerInfo {
    pub name: String,
    pub num_rounds: usize,
    pub winnings: usize,
}

/// Struct for the winner of a round
#[derive(Debug)]
pub struct RoundWinnerInfo {
    pub name: String,
    pub hand: Hand,
    pub winnings: usize,
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

    /// Add the hole cards to the community cards.
    fn add_hole_cards(&self, mut cards: Vec<Card>) -> Vec<Card> {
        let (c1, c2) = self.hole.unwrap();
        cards.push(c1);
        cards.push(c2);
        cards.clone()
    }

    pub fn place_bet(
        &mut self,
        call: usize,
        min: usize,
        community_cards: Vec<Card>,
        stage: Stage,
        cycle: u8,
    ) -> Option<Bet> {
        let cards = self.add_hole_cards(community_cards);
        let bet = self
            .actor
            .place_bet(
                call,
                min,
                stage,
                cycle,
                self.bank_roll,
                cards,
                self.hole.unwrap(),
            )
            .unwrap();
        match bet {
            Bet::Fold => Some(Bet::Fold),
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
        Some(strategy(call, min, bank_roll, cards, stage, cycle))
    }

    /// Accept a message and do nothing with it.
    fn update(&self, msg: &Msg) {}
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
