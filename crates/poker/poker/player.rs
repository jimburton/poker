/// Datatypes and functions for players in the game.
use crate::poker::betting_strategy;
use crate::poker::card::{Card, Hand};
use crate::poker::game::{Bet, Stage};
use std::fmt::Debug;
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

pub trait Player: Debug {
    /// Place a bet.
    fn place_bet(
        &mut self,
        call: usize,
        min: usize,
        community_cards: Vec<Card>,
        stage: Stage,
        _cycle: u8,
    ) -> Option<Bet>;

    /// Receive an update message, e.g. the status of the game or information about the
    /// winner of a round or game.
    fn update(&self, msg: &Msg) -> ();

    /// Pay the required amount to enter a round.
    fn ante_up(&mut self, ante: usize) -> Result<usize, &'static str>;

    /// Accept the pair of hole cards.
    fn accept_hole_cards(&mut self, hole_cards: Option<(Card, Card)>) -> ();

    /// Provide the player's name.
    fn get_name(&self) -> String;

    /// Provide the player's bank roll.
    fn get_bank_roll(&self) -> usize;

    /// True if the player has folded.
    fn get_folded(&self) -> bool;

    /// True if the player is all in.
    fn get_all_in(&self) -> bool;

    fn set_folded(&mut self, folded: bool);

    fn get_hole(&self) -> Option<(Card, Card)>;

    fn set_bank_roll(&mut self, bank_roll: usize);

    fn set_all_in(&mut self, all_in: bool);
}

/// The struct that represents a computer player.
#[derive(Debug, Clone)]
pub struct AutoPlayer {
    pub name: String,
    pub hole: Option<(Card, Card)>,
    pub bet: usize,
    pub bank_roll: usize,
    pub all_in: bool,
    pub folded: bool,
    pub betting_strategy: fn(usize, usize, usize, Vec<Card>, Stage, u8) -> Bet,
}

/// Implementation for the Player struct.
impl AutoPlayer {
    /// Construct a new Player.
    pub fn build(name: &str) -> Self {
        AutoPlayer {
            name: name.to_string(),
            bank_roll: 0,
            hole: None,
            bet: 0,
            all_in: false,
            folded: false,
            betting_strategy: betting_strategy::default_betting_strategy,
        }
    }

    /// Adopt a new strategy.
    pub fn set_betting_strategy(
        &mut self,
        strategy: fn(usize, usize, usize, Vec<Card>, Stage, u8) -> Bet,
    ) {
        self.betting_strategy = strategy;
    }

    ///
    /// Buy in to the game. Player is removed by Game if they don't buy in.
    pub fn buy_in(&mut self, buy_in: usize) -> Result<usize, &'static str> {
        if self.bank_roll >= buy_in {
            self.bank_roll -= buy_in;
            Ok(buy_in)
        } else {
            self.folded = true;
            Err("Can't join game.")
        }
    }

    fn add_hole_cards(&self, mut cards: Vec<Card>) -> Vec<Card> {
        let (c1, c2) = self.hole.unwrap();
        cards.push(c1);
        cards.push(c2);
        cards.clone()
    }
}

impl Player for AutoPlayer {
    /// Place a bet.
    fn place_bet(
        &mut self,
        call: usize,
        min: usize,
        community_cards: Vec<Card>,
        stage: Stage,
        cycle: u8,
    ) -> Option<Bet> {
        let strategy = self.betting_strategy;
        let cards = self.add_hole_cards(community_cards);
        match strategy(call, min, self.bank_roll, cards, stage, cycle) {
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

    fn update(&self, msg: &Msg) {}

    /// Buy in to a new round.
    fn ante_up(&mut self, ante: usize) -> Result<usize, &'static str> {
        if self.bank_roll > ante {
            self.bank_roll -= ante;
            Ok(ante)
        } else if self.bank_roll > 0 {
            let all_in_amount = self.bank_roll;
            self.bank_roll = 0;
            self.all_in = true;
            Ok(all_in_amount)
        } else {
            self.folded = true;
            Err("Can't join round.")
        }
    }

    /// Add the players hole cards to a list of cards.
    fn accept_hole_cards(&mut self, hole_cards: Option<(Card, Card)>) -> () {
        self.hole = hole_cards;
    }

    fn get_name(&self) -> String {
        self.name.to_string()
    }

    fn get_bank_roll(&self) -> usize {
        self.bank_roll
    }

    /// True if the player has folded.
    fn get_folded(&self) -> bool {
        self.folded
    }

    /// True if the player is all in.
    fn get_all_in(&self) -> bool {
        self.all_in
    }

    /// True if the player is all in.
    fn set_folded(&mut self, folded: bool) {
        self.folded = folded;
    }

    fn get_hole(&self) -> Option<(Card, Card)> {
        self.hole
    }

    fn set_bank_roll(&mut self, bank_roll: usize) {
        self.bank_roll = bank_roll;
    }

    fn set_all_in(&mut self, all_in: bool) {
        self.all_in = all_in;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build() {
        let player = AutoPlayer::build("James", 10_000);
        assert!(
            player.name == "James",
            "Expected new player to have name=='James', was {}",
            player.name
        );
    }
}
