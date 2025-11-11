use crate::server::actor::RemoteActor;
use axum::extract::ws::WebSocket;
use poker::{player::Player, poker};
use tokio::runtime::Handle;

pub async fn game_handler(player_name: String, socket: WebSocket, runtime_handle: Handle) {
    let actor = RemoteActor::build(socket, runtime_handle);
    let p = Player::build(&player_name, actor);
    let mut g = poker::new_game_one_player(p, 100, 3);
    g.play();
}
