# Texas Hold 'Em in Rust

The code is organised as follows:

+ the game logic in a [library](./crates/poker/README.md),
+ a [CLI client](./crates/poker_cli/README.md),
+ a [server](./crates/poker_server/README.md) exposing the library's
  API for websocket clients, and
+ a [web client](./web_client/README.md) written using React.

