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