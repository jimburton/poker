mod poker_cli;

use std::collections::HashMap;

use poker::poker::betting_strategy::{modest_betting_strategy, six_max};
use poker::poker::game::Game;
use poker::poker::player::AutoPlayer;
use poker_cli::player::CLIPlayer;
use poker_cli::start_one_player;

fn main() {
    let names: Vec<String> = vec![
        "James".to_string(),
        "Bob".to_string(),
        "Alice".to_string(),
        "Dileas".to_string(),
        "Evie".to_string(),
    ];
    let mut results: HashMap<String, (usize, usize)> = HashMap::new();
    for name in names {
        results.insert(name, (0, 0));
    }
    //for _ in (0..5).collect::<std::vec::Vec<i32>>() {
    let mut g: Game = start_one_player(100, 5);
    g.join(CLIPlayer::build("James"))
        .unwrap_or_else(|e| eprintln!("{e:?}"));
    g.join(AutoPlayer::build("Bob"))
        .unwrap_or_else(|e| eprintln!("{e:?}"));
    g.join(AutoPlayer::build("Alice"))
        .unwrap_or_else(|e| eprintln!("{e:?}"));
    let mut dileas = AutoPlayer::build("Dileas");
    dileas.betting_strategy = modest_betting_strategy;
    g.join(dileas);
    let mut evie = AutoPlayer::build("Evie");
    evie.betting_strategy = six_max;
    g.join(evie);

    let winner = g.play();
    println!("{:?}", winner);
    if let Some((won, winnings)) = results.get(&winner.name) {
        results.insert(winner.name, (won + 1, winnings + winner.winnings));
    } else {
        println!("Couldn't update results.");
    }
    //}
    //for (name, (won, winnings)) in results {
    //    println!("{} won {} games, total winnings {}", name, won, winnings);
    //}
}
