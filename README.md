# ikvs

**ikvs** (*Ikea Key-Value Store*... or *"I made a Key-Value Store"*, take your pick) is a small in-memory key-value store written in Rust, built for learning purposes.

It follows the excellent [**Build Redis from Scratch**](https://www.build-redis-from-scratch.dev/en/introduction) guide, which is written in Go — this is a from-scratch Rust port, following the same structure but adapted to Rust idioms (`std::net`, `std::io::BufReader`, threads instead of goroutines, etc).

## Status

This is a work in progress. Current progress against the guide:

| Chapter | Topic | Status |
|---|---|---|
| 1–2 | Introduction / First steps | ✅ |
| 3 | Building the server | ✅ TCP server, one thread per connection |
| 4 | Reading RESP | ✅ `Resp::read` parses arrays and bulk strings |
| 5 | Writing RESP | ⬜ not started |
| 6 | Redis commands (`PING`, `SET`, `GET`, `HSET`, ...) | ⬜ not started (`handler.rs` is a stub) |
| 7 | Data persistence (AOF) | ⬜ not started (`aof.rs` is a stub) |
| 8 | What's next | ⬜ |

Right now the server accepts connections, parses incoming RESP values, prints them, and replies with a hardcoded `+OK\r\n` to every request. No commands are actually executed yet.

## Project layout

```
src/
├── main.rs      # TCP listener, one thread per client connection
├── resp.rs      # RESP protocol parser (reading requests)
├── handler.rs   # command dispatch/execution (WIP)
└── aof.rs       # append-only file persistence (WIP)
```

## Running it

```sh
cargo run
```

The server listens on `127.0.0.1:6379` (the default Redis port).

You can talk to it with `redis-cli`, `nc`, or by sending a raw RESP array by hand:

```sh
redis-cli -p 6379 ping
```

```sh
printf '*1\r\n$4\r\nPING\r\n' | nc 127.0.0.1 6379
```

Since commands aren't implemented yet, every valid request currently just gets echoed to the server's stdout and answered with `+OK`.

## Why

This project exists to learn:

- How Redis-like databases work under the hood (RESP protocol, command dispatch, persistence).
- Rust's networking (`TcpListener`/`TcpStream`), I/O (`BufReader`), and concurrency (`std::thread`) primitives, by porting a Go implementation idea-for-idea.

## Reference

- Guide: [build-redis-from-scratch.dev](https://www.build-redis-from-scratch.dev/en/introduction) (Go)
- Protocol spec: [Redis Serialization Protocol (RESP)](https://redis.io/docs/latest/develop/reference/protocol-spec/)
