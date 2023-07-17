use http::Method;
use http::Request;
use server::Server; // use server struct via mod server

mod http; // copy http modules in http/mod.rs
mod server; // copy server.rs

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}
