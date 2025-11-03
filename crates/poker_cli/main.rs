mod poker_cli;

use poker::poker::betting_strategy::{modest_betting_strategy, six_max};
use poker::poker::game::Game;
use poker::poker::new_game_with_players;
use poker::poker::player::{AutoActor, Player};
use poker_cli::player::CLIPlayer;

fn main() {
    let players = vec![
        Player::build("James", CLIPlayer {}),
        Player::build("Bob", AutoActor::new()),
        Player::build("Alice", AutoActor::new()),
        Player::build("Dileas", AutoActor::build(modest_betting_strategy)),
        Player::build("Evie", AutoActor::build(six_max)),
    ];
    let mut g: Game = new_game_with_players(players, 100);

    let winner = g.play();
    println!("{}", winner);
    println!("{:?}", winner);
}
