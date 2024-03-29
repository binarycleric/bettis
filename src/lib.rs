#[macro_use]
extern crate log;
extern crate resp;
extern crate simple_logger;

pub mod server;
pub mod storage;
pub mod command;

use crate::server::Server;

pub fn start() {
    simple_logger::init().unwrap();

    let address = String::from("127.0.0.1");
    let port = String::from("7379");
    let server = Server::new(address, port);

    server.start();
}
