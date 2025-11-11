# Websocket server for the poker library.

See the [docs](../docs/poker_server/index.html).

Run the server with:

```
$ cargo run --bin poker_server
```

The server accepts several options:

```
$ cargo run --bin poker_server -- --help
Usage: target/debug/poker_server [options]

Options:
    -c, --config PATH   set the config file location
    -l, --log PATH      set the log file location
    -n, --host NAME     set the host name
    -p, --port NUMBER   set the port number
    -h, --help          print this help menu
```

The server can also be configured by making changes to the files
[poker.toml](conf/poker.toml) (which configures the hostname and port)
and [logging_conf.yaml](conf/logging_conf.yaml), then using the
command line flags to tell the server where the files are, or putting
them in the default location `~/conf/poker/`.

Initial axum/ws code from
[https://github.com/0Itsuki0/Rust_AxumWebSocket](https://github.com/0Itsuki0/Rust_AxumWebSocket).

```
$ npm install wscat
$ wscat -c ws://localhost:3000
Connected (press CTRL+C to quit)
> {"NewGame":{"name":"James"}}
< {"PlayerUpdate":{"player":"James","bet":{"Raise":200}}}
< {"General":{"msg":{"PlayersInfo":[["Player 2",9900],["Player 1",9950],["James",9900],["Player 3",9900]]}}}
< {"PlaceBet":{"args":{"call":0,"min":100,"stage":"PreFlop","cycle":0,"community_cards":[]},"hole_cards":[{"rank":"Jack","suit":"Hearts"},{"rank":"Rank2","suit":"Hearts"}],"bank_roll":9900,"best_hand":{"HighCard":"Jack"}}}
> {"PlayerBet":"Check"}
> {"PlayerBet":{"Raise":200}}
```
