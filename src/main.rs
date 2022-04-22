#![allow(dead_code)]

use server::Server;

mod server;
mod http;

fn main() {
    let address = String::from("127.0.0.1:9090");

    let server = Server::new(address);

    server.run();
}