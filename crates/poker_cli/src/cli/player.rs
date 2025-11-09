use poker::poker::{
    betting_strategy::BetArgs,
    card::Card,
    compare::best_hand,
    game::Bet,
    player::{Actor, Msg},
};

/// The struct that represents a CLI player.
#[derive(Debug, Clone)]
pub struct CLIPlayer {}

impl Actor for CLIPlayer {
    /// Place a bet.
    fn place_bet(
        &mut self,
        args: BetArgs,
        hole_cards: (Card, Card),
        bank_roll: usize,
    ) -> Option<Bet> {
        let mut cards = args.community_cards.clone();
        let (h1, h2) = (hole_cards.0, hole_cards.1);
        cards.push(h1);
        cards.push(h2);
        let bh = best_hand(&cards);

        println!("It's your turn to place a bet in the {}.", args.stage);
        println!("Hole cards: {}, {}", h1, h2);
        if !args.community_cards.is_empty() {
            println!("Community cards:",);
            args.community_cards.iter().for_each(|c| println!("{}", c));
        }
        println!(
            "The bet stands at {} (minimum amount to bet {})",
            args.call, args.min
        );
        println!("Bank roll: {}. Best hand: {}", bank_roll, bh);
        println!("Enter R(aise) <amount>, C(all), Ch(eck), A(ll in), F(old)");
        let mut input = String::new(); // A mutable String to hold the user input
        std::io::stdin()
            .read_line(&mut input) // Read input into the `input` variable
            .expect("Failed to read line");

        if let Some(bet) = parse_bet_string(input, bank_roll) {
            match bet {
                Bet::Fold => Some(Bet::Fold),
                Bet::Check => Some(Bet::Check),
                Bet::Call => Some(Bet::Call),
                Bet::Raise(n) => Some(Bet::Raise(n)),
                Bet::AllIn(n) => Some(Bet::AllIn(n)),
            }
        } else {
            None
        }
    }

    fn update(&self, msg: &Msg) {
        match msg {
            Msg::Bet { player, bet } => {
                println!("Player {} made bet: {}", player, bet);
            }
            Msg::PlayersInfo(players) => {
                println!(
                    "Playing this round with: {}",
                    players
                        .iter()
                        .map(|(player_name, bank_roll)| player_name.clone()
                            + " ("
                            + &bank_roll.to_string()
                            + ")")
                        .collect::<Vec<String>>()
                        .join(", "),
                );
            }
            Msg::Misc(contents) => {
                println!("Update: {}", contents,);
            }
            Msg::Game(w) => {
                println!("##############\n## {}.\n##############", w,);
            }
            Msg::RoundWinner(w) => {
                println!("##############\n## {}.\n##############", w,);
            }
            Msg::Round(stage) => {
                println!(
                    "##############\n## The {} stage is beginning.\n##############",
                    stage,
                );
            }
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
