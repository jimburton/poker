mod poker_cli;

use std::collections::HashMap;

use poker::poker::betting_strategy::{modest_betting_strategy, six_max};
use poker::poker::game::Game;
use poker::poker::player::{AutoActor, Player};
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
    g.join(Player::build("James", CLIPlayer {}))
        .unwrap_or_else(|e| eprintln!("{e:?}"));
    g.join(Player::build("Bob", AutoActor::new()))
        .unwrap_or_else(|e| eprintln!("{e:?}"));
    g.join(Player::build("Alice", AutoActor::new()))
        .unwrap_or_else(|e| eprintln!("{e:?}"));
    g.join(Player::build(
        "Dileas",
        AutoActor::build(modest_betting_strategy),
    ))
    .unwrap_or_else(|e| eprintln!("{e:?}"));
    g.join(Player::build("Evie", AutoActor::build(six_max)))
        .unwrap_or_else(|e| eprintln!("{e:?}"));

    let winner = g.play();
    println!("{:?}", winner);
    if let Some((won, winnings)) = results.get(&winner.name) {
        results.insert(winner.name, (won + 1, winnings + winner.winnings));
    } else {
        println!("Couldn't update results.");
    }
}
