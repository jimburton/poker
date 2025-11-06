use crate::server::actor::RemoteActor;
use axum::extract::ws::{Message, WebSocket};
use poker::poker::{game::Game, player::Player};

pub async fn game_handler(player_name: String, socket: WebSocket) -> Result<(), &'static str> {
    let actor = RemoteActor::build(socket);
    let p = Player::build(player_name.to_string(), actor).unwrap();
    let g = Game::new_game_one_player(p, 100, 3);
}
