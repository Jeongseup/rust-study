use crate::http::{Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::{Read, Write};
use std::net::TcpListener;
pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        // Server { addr: addr }
        Server { addr }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap(); // if got an error, triger to panic for stopping runtime

        loop {
            match listener.accept() {
                Err(e) => println!("Failed to establish a conncetion: {}", e),

                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024]; // memory initialize before using the array
                    match stream.read(&mut buffer) {
                        // read by using buffer through copy data from socket to buffer
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                    Response::new(
                                        StatusCode::Ok,
                                        Some("<h1>Hello </h1>".to_string()),
                                    )
                                    // response.send(&mut stream);
                                }
                                Err(e) => {
                                    println!("Faield to parse err: {}", e);
                                    Response::new(StatusCode::BadRequest, None)
                                }
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Faield to send response {}", e);
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                    println!("OK");
                }
            }
        }
    }
}
