# Help!

Can you help me explain why messages on this Unix socket between two processes are sometimes dropped?

To demonstrate the problem run

```
cargo run --bin test_server &
cargo run --bin test_client
```

After a short while both processes will panic. Can you explain why?

# What's going on

`test_server` starts listening on a unix socket, when the first client connects its runs the `test_peer_loop` (see below) to handle the connection.

`test_client` connects to the unix socket opened by `test_server`. Once connected it runs the `test_peer_loop` (see below) to handle the connection.

Both ends of the unix socket connection act the same. They run the function `test_peer_loop`, which behaves thusly:

- Every 5ms a message with a unique incrementing intenger ID and a payload of some bytes is sent to the remote peer.
- Whenever a message is received from the remote peer a check is made that the ID is the next expected ID. If not the peer panics.

# Expected Behaviour

No panic, messages sent continuously in both directions.

# Actual Behaviour

After a random number of successful messages either the client or server will receive an out-of-order message caused by having somehow never received one message. I can not work out how this is possible. Any help greatly apprecaited!