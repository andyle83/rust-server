use std::io::{Read, Write};
use std::net::{TcpListener};

use crate::http::{Request, Response, StatusCode};
use std::convert::TryFrom;
use crate::http::request::ParseError;

// trait can have default handle
pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(
            StatusCode::BadRequest,
            None
        )
    }
}


pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    pub fn run(self, mut handle: impl Handler) {
        println!("Server is running on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, address)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    // dbg!(request);
                                    // Response::new(
                                    //     StatusCode::Ok,
                                    //     Some("<h1 style='color:orange;'>BO DEP TRAI</h1><h3>Me xinh gai</h3>".to_uppercase())
                                    // )
                                    handle.handle_request(&request)
                                },
                                Err(err) => {
                                    // println!("Failed to parse : {}", err);
                                    // Response::new(
                                    //     StatusCode::BadRequest,
                                    //     None
                                    // )
                                    handle.handle_bad_request(&err)
                                }
                            };

                            if let Err(e) = response.send(&mut stream) {
                                print!("Failed to send the response {}", e);
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