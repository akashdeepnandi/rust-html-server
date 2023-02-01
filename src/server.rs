use std::{io::Read, net::TcpListener, process};

use crate::http::{Request, Response, StatusCode, ParseError};

pub struct Server {
    addr: String,
}

pub trait Handler {
    fn handle_request(&self, req: &Request) -> Response;
    fn handle_bad_request(&self, e: &ParseError) -> Response {
        print!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

impl Server {
    pub fn new(addr: String) -> Self {
        Server { addr }
    }

    pub fn run(self, handler: impl Handler) {
        println!("Listening on {}", self.addr);
        let listener = TcpListener::bind(self.addr).unwrap_or_else(|e| {
            println!("Error: {e}");
            process::exit(1);
        });

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buf = [0; 1024];
                    match stream.read(&mut buf) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buf));
                            let res = match Request::try_from(&buf[..]) {
                                Ok(req) => handler.handle_request(&req),
                                Err(e) => handler.handle_bad_request(&e),
                            };
                            if let Err(e) = res.send(&mut stream) {
                                print!("Failed to send response: {e}")
                            }
                            // let res: &Result<Request, _> = &buf[..].try_into();
                        }
                        Err(e) => println!("Failed to read  from connection: {e}"),
                    }
                }
                Err(e) => println!("Failed to establish a connection: {e}"),
            }
        }
    }
}
