use crate::http::Request;
use std::convert::TryFrom;
use std::io::Read;
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

                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                }
                                Err(e) => println!("Faield to parse err: {}", e),
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
