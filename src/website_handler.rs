use super::server::Handler;
use super::http::{
    Request, 
    Response, 
    StatusCode, 
    Method, 
    headers::{ContentType, HeadersReq}
};
use std::fs;


pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<(String, u64)> {
        let path = format!("{}/{}", self.public_path, file_path);

        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    let content_length = fs::metadata(&path).unwrap().len();
                    let content = fs::read_to_string(path).unwrap();

                    Some(( content, content_length ))
                } else {
                    println!("Directory Traversal Attack Attempted: {}", file_path);
                    None
                }
            },
            Err(_) => None
        }
    }

    fn create_response(&self, file_path: &str) -> Response {
        let ( content, content_length ) = self.read_file(file_path).unwrap();
        let mut file_type = ContentType::NoType;

        if let Some(i) = file_path.find('.') {
            let ext = &file_path[i + 1..];

            file_type = match ext {
                "html" => ContentType::HTML,
                "css" => ContentType::CSS,
                _ => ContentType::NoType
            }
        }
        Response::new(StatusCode::Ok, Some(content), Some(content_length), file_type)
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => self.create_response("index.html"),
                "/hello" => self.create_response("hello.html"),

                path => match self.read_file(path) {
                    Some(_) => self.create_response(path),
                    None => Response::new(StatusCode::NotFound, None, None, ContentType::NoType)
                }
            }
            _ => Response::new(StatusCode::NotFound, None, None, ContentType::NoType)
        }
    }
}