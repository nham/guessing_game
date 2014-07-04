# Guessing Game TCP

This is Steve Klabnik's "guessing game" in Rust redone as a TCP client/server.

When a client connects to the server, the server generates a new random number and handles guesses until the correct guess is submitted.

To install:

```bash
$ git clone https://github.com/nham/guessing_game
$ cd guessing_game
$ cargo build
$ ./target/guessing_game_tcp_server
$ ./target/guessing_game_tcp_client
```

You must have both Rust and Cargo installed.
