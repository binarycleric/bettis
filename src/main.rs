extern crate quarrie;

use quarrie::network::Server;
use quarrie::DataTable;

fn main() {
    let mut data_table = DataTable::new(15);
    let server = Server::new("127.0.0.1", "6379");

    server.start(&mut data_table);
}
