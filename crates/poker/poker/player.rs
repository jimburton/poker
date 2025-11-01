/// Datatypes and functions for players in the game.
use crate::poker::betting_strategy;
use crate::poker::card::{Card, Hand};
use crate::poker::game::{Bet, Stage};

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
}

/// The struct that represents a player.
#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub hole: Option<(Card, Card)>,
    pub bet: usize,
    pub bank_roll: usize,
    pub all_in: bool,
    pub folded: bool,
    pub betting_strategy: fn(usize, usize, usize, Vec<Card>, Stage, u8) -> Bet,
    pub interactive: bool,
}

/// Implementation for the Player struct.
impl Player {
    /// Construct a new Player.
    pub fn build(name: &str, bank_roll: usize) -> Self {
        Player {
            name: name.to_string(),
            bank_roll,
            hole: None,
            bet: 0,
            all_in: false,
            folded: false,
            betting_strategy: betting_strategy::default_betting_strategy,
            interactive: false,
        }
    }

    /// Adopt a new strategy.
    pub fn set_betting_strategy(
        &mut self,
        strategy: fn(usize, usize, usize, Vec<Card>, Stage, u8) -> Bet,
    ) {
        self.betting_strategy = strategy;
    }

    pub fn place_bet_interactive(
        &mut self,
        call: usize,
        min: usize,
        community_cards: Vec<Card>,
        stage: Stage,
        _cycle: u8,
    ) -> Option<Bet> {
        println!("It's your turn to place a bet in the {:?}.", stage);
        println!(
            "Hole cards:\n{:?}\n{:?}",
            self.hole.unwrap().0,
            self.hole.unwrap().1
        );
        if !community_cards.is_empty() {
            println!("Community cards:",);
            community_cards.iter().for_each(|c| println!("{:?}", c));
        }
        println!("The bet stands at {} (minimum amount to bet {})", call, min);
        println!("Bank roll: {}", self.bank_roll);
        println!("Enter R(aise) <amount>, C(all), Ch(eck), A(ll in), F(old)");
        let mut input = String::new(); // A mutable String to hold the user input
        std::io::stdin()
            .read_line(&mut input) // Read input into the `input` variable
            .expect("Failed to read line");

        if let Some(bet) = parse_bet_string(input, self.bank_roll) {
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
        } else {
            None
        }
    }

    /// Place a bet.
    pub fn place_bet(
        &mut self,
        call: usize,
        min: usize,
        community_cards: Vec<Card>,
        stage: Stage,
        cycle: u8,
    ) -> Bet {
        let strategy = self.betting_strategy;
        let cards = self.add_hole_cards(community_cards);
        match strategy(call, min, self.bank_roll, cards, stage, cycle) {
            Bet::Fold => Bet::Fold,
            Bet::Check => Bet::Check,
            Bet::Call => {
                self.bank_roll -= call;
                Bet::Call
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

    pub fn update(&self, msg: &Msg) {
        match msg {
            Msg::MsgBet(update) => {
                println!(
                    "Update: Player {} made bet: {:?}",
                    update.player, update.bet
                );
            }
            Msg::MsgMisc(contents) => {
                println!("Update: {}", contents,);
            }
        }
    }

    /// Add the players hole cards to a list of cards.
    fn add_hole_cards(&self, mut cards: Vec<Card>) -> Vec<Card> {
        let (h1, h2) = self.hole.unwrap();
        cards.push(h1);
        cards.push(h2);
        cards.clone()
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
}

fn parse_bet_string(input: String, all_in_amount: usize) -> Option<Bet> {
    let parts: Vec<&str> = input.trim().split(" ").collect();
    if parts.len() == 2 {
        let amount: usize = parts[1]
            .trim() // Remove whitespace
            .parse() // Convert to i32
            .expect("Please enter a valid number");
        Some(Bet::Raise(amount))
    } else {
        match parts[0] {
            "C" => Some(Bet::Call),
            "Ch" => Some(Bet::Check),
            "F" => Some(Bet::Fold),
            "A" => Some(Bet::AllIn(all_in_amount)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
