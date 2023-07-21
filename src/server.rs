use crate::http::{headers::ContentType, ParseError, Request, Response, StatusCode};

use std::{
    any::Any,
    convert::TryFrom,
    io::Read,
    net::TcpListener,
    sync::{Arc, Mutex},
    thread,
};

pub trait Handler: Send + Sync {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None, None, ContentType::NoType)
    }

    fn print_req_headers(request: &Request);
}

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self, handler: impl Handler + 'static) {
        println!("Listening on {}", self.addr);
        let handler_arc = Arc::new(Mutex::new(handler));

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];

                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            let handler_clone = Arc::clone(&handler_arc);

                            let join_handle = thread::spawn(move || {
                                println!(
                                    "Received a request: {}",
                                    String::from_utf8_lossy(&buffer)
                                );
                                let mut handler = handler_clone.lock().unwrap();

                                match Request::try_from(&buffer[..]) {
                                    Ok(request) => handler.handle_request(&request),
                                    Err(e) => handler.handle_bad_request(&e),
                                }
                            });

                            match join_handle.join() {
                                Ok(response) => {
                                    if let Err(e) = response.send(&mut stream) {
                                        println!("Failed to parse a request: {}", e);
                                    }
                                }
                                Err(e) => {
                                    println!("Failed to parse a request: {}", Self::format_any(&e));
                                }
                            };
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => println!("Connection failed: {}", e),
            }
        }
    }

    fn format_any(value: &Box<dyn Any + Send>) -> String {
        format!("{:?}", value)
    }
}
