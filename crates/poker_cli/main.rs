mod poker_cli;

use poker::poker::betting_strategy::{modest_betting_strategy, six_max};
use poker::poker::game::Game;
use poker::poker::player::{AutoActor, Player};
use poker_cli::player::CLIPlayer;
use poker_cli::start_one_player;

fn main() {
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
    println!("{}", winner);
    println!("{:?}", winner);
}
