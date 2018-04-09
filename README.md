# rust-tcp-fun
Trying to reimplement some basic redis features in Rust.

## What is this even?

I'm trying to learn a bit more about using Rust for systems programming so I'd 
like to build a basic key/value store similar to Redis. Ideally, it'll be protocol
compatible with a handful of commands but use a different storage backend.

I don't intend of making this a "real" project, this is just a teaching exercise.

## Goals

* Built a simple service that is protocol compatible with commands redis.
* Have some kind of persistance and threading to utilize multiple CPUs.
* Minimize the use of third party libraries and mainly rely on Rust's stdlib.
* Not be taken too seriously.

## What works?

* Basic GET/SET commands with 32-bit integers.

## What doesn't work?

* Everything else.

### No really?

* Commands are single-threaded.
* Only 32-bit integers are supported.
* Configuration is hard-coded to run on localhost with port 3000.
* Storage is in-memory only.

What do you expect? This is a learning project.