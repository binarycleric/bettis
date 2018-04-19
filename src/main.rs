extern crate quarrie;

use quarrie::tcp::Server;
use quarrie::storage::DataTable;

fn main() {
    let mut _data_table = DataTable::new();
    let server = Server::new("127.0.0.1", "6379");

    server.start();
}
