#![allow(dead_code)]

use server::Server;
use crate::website_handler::WebsiteHandler;
use std::env;

mod server;
mod http;
mod website_handler;

fn main() {
    // read environment at run-time
    let default_path = format!("{}/public",env!("CARGO_MANIFEST_DIR"));
    let address = String::from("127.0.0.1:9090");
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    let handle = WebsiteHandler::new(public_path);

    let server = Server::new(address);

    server.run(handle);
}