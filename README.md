# Texas Hold 'Em in Rust

The code is organised as follows:

+ the game logic in a [library](./crates/poker/README.md) ([docs](docs/poker/index.html)),
+ a [CLI client](./crates/poker_cli/README.md) ([docs](docs/poker_cli/index.html)),
+ a [server](./crates/poker_server/README.md) exposing the library's
  API for websocket clients ([docs](docs/poker_server/index.html)),
+ a [web client](./web_client/README.md) written using React.

