use crate::poker::card::{Card, Hand};
use crate::poker::game::Bet;

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub hole: Option<(Card, Card)>,
    pub bet: usize,
    pub bank_roll: usize,
    pub all_in: bool,
    pub folded: bool,
    pub betting_strategy: fn(usize, usize, usize, Vec<Card>) -> Bet,
}

#[derive(Debug, Clone)]
pub struct PlayerHand {
    pub name: String,
    pub best_hand: Hand,
    pub cards: Vec<Card>,
}

impl Player {
    /// Construct a new Player struct.
    pub fn build(name: &str, bank_roll: usize) -> Self {
        Player {
            name: name.to_string(),
            bank_roll,
            hole: None,
            bet: 0,
            all_in: false,
            folded: false,
            betting_strategy: Player::default_betting_strategy,
        }
    }

    /// Default betting strategy, which does the following:
    ///
    /// + fold if necessary,
    /// + goes all in if neccessary,
    /// + check if possible,
    /// + call the bet.
    fn default_betting_strategy(
        call: usize,
        _min: usize,
        bank_roll: usize,
        _cards: Vec<Card>,
    ) -> Bet {
        if bank_roll == 0 {
            Bet::Fold
        } else if bank_roll <= call {
            Bet::AllIn(bank_roll)
        } else if call == 0 {
            Bet::Check
        } else {
            Bet::Call(call)
        }
    }

    /// Default betting strategy, which does the following:
    ///
    /// + fold if necessary,
    /// + goes all in if neccessary,
    /// + bet the minimum amount if currently zero,
    /// + call the bet.
    fn modest_betting_strategy(
        call: usize,
        min: usize,
        bank_roll: usize,
        _cards: Vec<Card>,
    ) -> Bet {
        if bank_roll == 0 {
            Bet::Fold
        } else if bank_roll <= call {
            Bet::AllIn(bank_roll)
        } else if call == 0 {
            Bet::Raise(min)
        } else {
            Bet::Call(call)
        }
    }

    pub fn set_betting_strategy(&mut self, strategy: fn(usize, usize, usize, Vec<Card>) -> Bet) {
        self.betting_strategy = strategy;
    }

    /// Place a bet.
    pub fn place_bet(&mut self, call: usize, min: usize, community_cards: Vec<Card>) -> Bet {
        let strategy = self.betting_strategy;
        match strategy(call, min, self.bank_roll, community_cards) {
            Bet::Fold => Bet::Fold,
            Bet::Check => Bet::Check,
            Bet::Call(n) => {
                self.bank_roll -= n;
                Bet::Call(n)
            }
            Bet::Raise(n) => {
                self.bank_roll -= n;
                Bet::Raise(n)
            }
            Bet::AllIn(n) => {
                self.bank_roll = 0;
                Bet::AllIn(n)
            }
        }
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
    ///
    /// Buy in to a new round.
    pub fn ante_up(&mut self, ante: usize) -> Result<usize, &'static str> {
        if self.bank_roll >= ante {
            self.bank_roll -= ante;
            Ok(ante)
        } else {
            self.folded = true;
            Err("Can't join round.")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::poker::card::{Card, Hand, Rank, Suit};

    #[test]
    fn test_build() {
        let player = Player::build("James", 10_000);
        assert!(
            player.name == "James",
            "Expected new player to have name=='James', was {}",
            player.name
        );
    }
}
