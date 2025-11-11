use crate::poker::{
    betting_strategy,
    betting_strategy::{BetArgs, BettingStrategy},
    card::Card,
    game::Bet,
    player::{Actor, Msg},
};

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
    /// Stub to accept the name and bank roll at the beginning of the game.
    fn set_name_and_bank_roll(&self, _name: &String, _bank_roll: usize) {}

    /// Stun to accept the hole cards.
    fn hole_cards(&self, _hole_cards: (Card, Card)) {}

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
