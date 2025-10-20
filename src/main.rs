use poker::poker::game::Game;

fn main() {
    let mut g: Game = Game::build(10000, 5);
    g.join("James").unwrap_or_else(|e| eprintln!("{e:?}"));
    g.join("Bob").unwrap_or_else(|e| eprintln!("{e:?}"));
    g.join("Alice").unwrap_or_else(|e| eprintln!("{e:?}"));
    g.join("Dileas").unwrap_or_else(|e| eprintln!("{e:?}"));

    g.play();
}
