use std::io::{Read, Write};
use std::net::{TcpListener};

use crate::http::Request;
use std::convert::TryFrom;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    pub fn run(self) {
        println!("Server is running on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, address)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                    write!(stream, "HTTP/1.1 404 Not Found\r\n\r\n");
                                },
                                Err(err) => println!("Failed to parse : {}", err)
                            }
                        },
                        Err(err) => println!("Failed to read from connection {}", err)
                    }
                },
                Err(e) => println!("Failed to establish connection {:?}", e),
            }
        }
    }
}