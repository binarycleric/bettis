# rust-tcp-fun

Trying to reimplement some basic redis features in Rust.

**TODO**: Come up with a real name for this project.

## What is this even?

I'm trying to learn a bit more about using Rust for systems programming so I'd 
like to build a basic key/value store similar to Redis. Ideally, it'll be protocol
compatible with a handful of commands but use a different storage backend.

## Goals

* Built a simple service that is protocol compatible with (some/most) redis commands.
* Threading to make use of servers with multiple CPUs.
* (Eventually) Persistance and streaming replication.
* Backup and recovery.
* Minimize the use of third party libraries and mainly rely on Rust's stdlib.

## What works?

* Some basic redis protocol value parsing.
* Running a basic select/set/get script using Ruby's redis client.

## Development

This is pretty basic right now.

Run tests.

```bash
$ cargo test
```

Start the service for basic testing

```bash
$ cargo run
```
