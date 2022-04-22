#![allow(dead_code)]

use server::Server;
use crate::website_handler::WebsiteHandler;

mod server;
mod http;
mod website_handler;

fn main() {
    let address = String::from("127.0.0.1:9090");
    let handle = WebsiteHandler;

    let server = Server::new(address);

    server.run(handle);
}