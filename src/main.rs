use poker::poker::game::Game;

fn main() {
    let mut g: Game = Game::build(100, 20, 40, 4);
    g.add_player("James", 1000)
        .unwrap_or_else(|e| eprintln!("{e:?}"));
    g.add_player("Bob", 1000)
        .unwrap_or_else(|e| eprintln!("{e:?}"));
    g.add_player("Alice", 1000)
        .unwrap_or_else(|e| eprintln!("{e:?}"));
    g.add_player("Dileas", 1000)
        .unwrap_or_else(|e| eprintln!("{e:?}"));

    g.play();
}
