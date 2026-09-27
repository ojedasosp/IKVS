# ikvs

**ikvs** (*Ikea Key-Value Store*... or *"I made a Key-Value Store"*, take your pick) is a small in-memory key-value store written in Rust, built for learning purposes.

It follows the excellent [**Build Redis from Scratch**](https://www.build-redis-from-scratch.dev/en/introduction) guide, which is written in Go — this is a from-scratch Rust port, following the same structure but adapted to Rust idioms (`std::net`, `std::io::BufReader`, threads instead of goroutines, etc).

## Status

This is a work in progress. Current progress against the guide:

| Chapter | Topic | Status |
|---|---|---|
| 1–2 | Introduction / First steps | ✅ |
| 3 | Building the server | ✅ TCP server, one thread per connection |
| 4 | Reading RESP | ✅ `Reader::read` parses arrays and bulk strings |
| 5 | Writing RESP | ✅ `Writer::write` marshals strings, bulks, arrays, errors, and null |
| 6 | Redis commands (`PING`, `SET`, `GET`, `HSET`, `HGET`, `HGETALL`) | ✅ dispatched through a command table in `handler.rs` |
| 7 | Data persistence (AOF) | ⬜ not started (`aof.rs` is a stub) |
| 8 | What's next | ⬜ |

The server accepts connections, parses incoming RESP requests, and dispatches them to real command handlers backed by an in-memory, thread-safe store (`RwLock<HashMap<...>>`). Unknown commands get a proper RESP error reply. Data is not persisted yet — everything lives in memory and is lost on restart.

## Project layout

```
src/
├── main.rs      # TCP listener, one thread per client connection
├── resp.rs      # RESP protocol: reading requests, writing replies
├── handler.rs   # command table + in-memory DB/HDB (thread-safe via RwLock)
└── aof.rs       # append-only file persistence (WIP, empty stub)
```

## Supported commands

| Command | Description |
|---|---|
| `PING [message]` | Replies with `PONG`, or echoes `message` if given |
| `SET key value` | Stores `value` under `key` in the string store |
| `GET key` | Returns the value stored under `key`, or an error if missing |
| `HSET hash key value` | Stores `key`/`value` inside the hash named `hash` |
| `HGET hash key` | Returns the value for `key` inside `hash`, or null if missing |
| `HGETALL hash` | Returns all key/value pairs stored in `hash` as a flat array |

## Running it

```sh
cargo run
```

The server listens on `127.0.0.1:6379` (the default Redis port).

You can talk to it with `redis-cli`, `nc`, or by sending a raw RESP array by hand:

```sh
redis-cli -p 6379 ping
redis-cli -p 6379 set foo bar
redis-cli -p 6379 get foo
redis-cli -p 6379 hset myhash field1 value1
redis-cli -p 6379 hgetall myhash
```

```sh
printf '*1\r\n$4\r\nPING\r\n' | nc 127.0.0.1 6379
```

## Why

This project exists to learn:

- How Redis-like databases work under the hood (RESP protocol, command dispatch, persistence).
- Rust's networking (`TcpListener`/`TcpStream`), I/O (`BufReader`), and concurrency (`std::thread`) primitives, by porting a Go implementation idea-for-idea.

## Reference

- Guide: [build-redis-from-scratch.dev](https://www.build-redis-from-scratch.dev/en/introduction) (Go)
- Protocol spec: [Redis Serialization Protocol (RESP)](https://redis.io/docs/latest/develop/reference/protocol-spec/)
