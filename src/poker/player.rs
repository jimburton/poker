use crate::poker::card::Card;
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
        min: usize,
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
    /// Buy in to a new round.
    pub fn ante_up(&mut self, buy_in: usize) -> Result<usize, &'static str> {
        if self.bank_roll >= buy_in {
            self.bank_roll -= buy_in;
            Ok(buy_in)
        } else {
            self.folded = true;
            Err("Can't join game.")
        }
    }
}
