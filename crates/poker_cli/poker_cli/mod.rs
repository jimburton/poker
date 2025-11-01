pub mod player;
use poker::poker::game::Game;

pub fn start_one_player(big_blind: usize, num_players: u8) -> Game {
    Game::build(big_blind, num_players)
}

// You can also declare sub-modules here if you have more files in this directory.
// mod sub_module_file;
