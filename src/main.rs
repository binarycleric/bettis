extern crate quarrie;

use quarrie::tcp::Server;
use quarrie::storage::DataTable;

use std::thread;
use std::sync::mpsc;

fn main() {
    let mut data_table = DataTable::new();
    let server = Server::new("127.0.0.1", "6379");

    server.start(&mut data_table);
}
