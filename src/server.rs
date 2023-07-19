use std::io::Read;
use crate::http::{
    Request, 
    Response, 
    StatusCode, 
    ParseError, 
    headers::ContentType
};
use std::convert::TryFrom;
use std::net::TcpListener;
use std::sync::Mutex;
use std::thread;


pub trait Handler {
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

    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {     
                    let mut buffer = [0; 1024];  

                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            let joinHandle = thread::spawn(move || {
                                println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                                match Request::try_from(&buffer[..]) {
                                    Ok(request) => handler.handle_request(&request),
                                    Err(e) => handler.handle_bad_request(&e),            
                                }
                            });
                            let response = joinHandle.join()?;
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to parse a request: {}", e);
                            }
                        },
                        Err(e) => println!("Failed to read from connection: {}", e)
                    }
                },
                Err(e) => println!("Connection failed: {}", e)
            }
        }
    }
}