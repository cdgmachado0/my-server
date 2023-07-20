use std::io::Read;
use crate::http::response;
use crate::http::{
    Request, 
    Response, 
    StatusCode, 
    ParseError, 
    headers::ContentType
};
use std::convert::TryFrom;
use std::net::TcpListener;
use std::thread;
use core::fmt::Display;
use std::any::Any;
// use std::marker::Send;
use std::fmt;
use std::sync::{Arc, Mutex};


pub trait Handler: Send + Sync {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None, None, ContentType::NoType)
    }

    fn print_req_headers(request: &Request);
}

pub struct Server {
    addr: String
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
                                    println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                                    let mut handler = handler_clone.lock().unwrap();
    
                                    match Request::try_from(&buffer[..]) {
                                        Ok(request) => handler.handle_request(&request),
                                        Err(e) => handler.handle_bad_request(&e),            
                                    }
                                }
                            );  

                            match join_handle.join() {
                                Ok(response) => {
                                    response.send(&mut stream);
                                },
                                Err(e) => {
                                    println!("Failed to parse a request: {}", Self::format_any(&e));
                                }
                            };
                        },
                        Err(e) => println!("Failed to read from connection: {}", e)
                    }
                },
                Err(e) => println!("Connection failed: {}", e)
            }
        }
    }

    fn format_any(value: &Box<dyn Any + Send>) -> String {
        format!("{:?}", value)
    }
}
