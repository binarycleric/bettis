extern crate quarrie;
extern crate resp;

use resp::{Value, encode, encode_slice, Decoder};

use quarrie::tcp::Listener;
use quarrie::storage::Database;

fn main() {
    let mut data_table = Database::new();
    let server = Listener::new("127.0.0.1", "6379");

    server.start(&mut data_table);
}
