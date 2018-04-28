# Bettis

Basic key/value store that is protocol compatible with Redis.

[![Build Status](https://travis-ci.org/binarycleric/bettis.svg?branch=master)](https://travis-ci.org/binarycleric/bettis)

This project is **experimental** and still under active development. Pull
requests and bugs are welcome but I'm not really interested in feature requests
right now. If you like this project and would like to implement new things, please
talk to me (@binarycleric on Twitter) and we can make a plan!

### Goals

* (Mostly) protocol compatible with Redis.
* Can take advantage of multiple CPUs for scaling.
* Streaming replication for followers.

## Notes

* `SELECT` command is accepted but is currently ignored. All connections go to database 0.
