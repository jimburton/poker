use poker::poker::types::Game;

fn main() {
    let mut g: Game = Game::build(100, 3);
    g.add_player("James", 1000)
        .unwrap_or_else(|e| eprintln!("{e:?}"));
    g.add_player("Bob", 1000)
        .unwrap_or_else(|e| eprintln!("{e:?}"));
    g.add_player("Alice", 1000)
        .unwrap_or_else(|e| eprintln!("{e:?}"));
    g.add_player("Dileas", 1000)
        .unwrap_or_else(|e| eprintln!("{e:?}"));

    g.play();

    println!("Cards in deck: {:?}", g.deck.len());
    println!("Players: {:?}", g.players);
}
