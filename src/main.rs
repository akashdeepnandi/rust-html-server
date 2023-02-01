#![allow(dead_code)]
use std::env;

use server::Server;

mod server;
mod http;
mod web_handler;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);

    println!("Public Path: {public_path}");
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(web_handler::WebHandler::new(public_path));
    println!("Hello, world!");
}
