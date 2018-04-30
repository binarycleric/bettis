#[macro_use]
extern crate log;
extern crate resp;
extern crate simple_logger;

mod tcp;
pub mod storage;
mod command;

use tcp::Listener;
use resp::Value as RespValue;
pub use storage::Database;

pub fn integer(int: i64) -> RespValue {
    RespValue::Integer(int)
}

pub fn bulk_string<'a>(string: &'a str) -> RespValue {
    RespValue::Bulk(string.to_string())
}

pub fn start() {
    simple_logger::init().unwrap();

    let mut data_table = Database::new();
    let server = Listener::new("127.0.0.1", "6379");

    server.start(&mut data_table);
}
