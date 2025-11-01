use poker::poker::{
    card::Card,
    game::{Bet, Stage},
    player::{Msg, Player},
};

/// The struct that represents a CLI player.
#[derive(Debug, Clone)]
pub struct CLIPlayer {
    pub name: String,
    pub hole: Option<(Card, Card)>,
    pub bet: usize,
    pub bank_roll: usize,
    pub all_in: bool,
    pub folded: bool,
}

/// Implementation for the Player struct.
impl CLIPlayer {
    /// Construct a new Player.
    pub fn build(name: &str) -> Self {
        CLIPlayer {
            name: name.to_string(),
            bank_roll: 0,
            hole: None,
            bet: 0,
            all_in: false,
            folded: false,
        }
    }

    fn add_hole_cards(&self, mut cards: Vec<Card>) -> Vec<Card> {
        let (c1, c2) = self.hole.unwrap();
        cards.push(c1);
        cards.push(c2);
        cards.clone()
    }
}

impl Player for CLIPlayer {
    /// Place a bet.
    fn place_bet(
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

    fn update(&self, msg: &Msg) {
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
            Msg::MsgWinner(w) => {
                println!("Update: {:?}", w,);
            }
        }
    }

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
    fn accept_hole_cards(&mut self, hole_cards: Option<(Card, Card)>) {
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

    fn set_all_in(&mut self, all_in: bool) {
        self.all_in = all_in;
    }

    fn set_bank_roll(&mut self, bank_roll: usize) {
        self.bank_roll = bank_roll;
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
