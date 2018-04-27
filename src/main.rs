extern crate bettis;

use bettis::tcp::Listener;
use bettis::storage::Database;

fn main() {
    let mut data_table = Database::new();
    let server = Listener::new("127.0.0.1", "6379");

    server.start(&mut data_table);
}
