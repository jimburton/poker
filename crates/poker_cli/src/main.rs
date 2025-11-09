mod cli;

use cli::player::CLIPlayer;
use poker::poker::{
    autoactor::AutoActor,
    betting_strategy::{modest_betting_strategy, six_max},
    game::Game,
    new_game_with_players,
    player::Player,
};

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
