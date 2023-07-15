use super::server::Handler;
use super::http::{Request, Response, StatusCode, Method};
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
                    // if let Ok(metadata) = fs::metadata(path) {
                    //     let content_length = metadata.len();

                    //     Some((
                    //         fs::read_to_string(path)?,
                    //         content_length
                    //     ))
                    // }

                    // let content_length = fs::metadata(path)?;
                    // let size = content_length.len();

                    // // unimplemented!();
                    // let content = fs::read_to_string(path)?;

                    // Some((content, size))

                    let metadata = fs::metadata(path).unwrap();


                    unimplemented!()
                 
                } else {
                    println!("Directory Traversal Attack Attempted: {}", file_path);
                    None
                }
                // fs::read_to_string(path).ok()
            },
            Err(_) => None
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/hello" => Response::new(StatusCode::Ok, self.read_file("hello.html")),

                path => match self.read_file(path) {
                    Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                    None => Response::new(StatusCode::NotFound, None)
                }
            }
            _ => Response::new(StatusCode::NotFound, None)
        }
    }
}