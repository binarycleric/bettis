#[macro_use]
extern crate log;
extern crate simple_logger;

mod tcp;
mod storage;
mod commands;

use tcp::Listener;
use storage::Database;

pub fn start() {
    simple_logger::init().unwrap();

    let mut data_table = Database::new();
    let server = Listener::new("127.0.0.1", "6379");

    server.start(&mut data_table);
}

// TODO: Redis protocol parser.
// https://redis.io/topics/protocol
// TODO: Need to keep reading from the stream until the connection is closed.
/*

    For Simple Strings the first byte of the reply is "+"
    For Errors the first byte of the reply is "-"
    For Integers the first byte of the reply is ":"
    For Bulk Strings the first byte of the reply is "$"
    For Arrays the first byte of the reply is "*"


            *2\r\n$6\r\nselect\r\n$2\r\n15\r\n
            [ select 15 ]



let command_string = str::from_utf8(&buffer).unwrap();
let mut command_list = command_string.split("\r\n");

println!("{:?}", command_list);

let command_size = command_list.next().unwrap();
println!("{:?}", command_size);

*/
