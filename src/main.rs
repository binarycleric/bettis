extern crate quarrie;

use quarrie::tcp::Server;
use quarrie::storage::Database;

fn main() {
    let mut data_table = Database::new();
    let server = Server::new("127.0.0.1", "6379");

    server.start(&mut data_table);
}
