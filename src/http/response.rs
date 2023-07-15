use std::io::{Write, Result as IoResult};
use super::StatusCode;
use chrono::prelude::{DateTime, Utc};
use std::env::consts::{OS, ARCH};

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
    server: String, //--mine
    content_type: String, 
    content_length: Option<u64>,
    date: DateTime<Utc>
}

impl Response {
    pub fn new(
        status_code: StatusCode, 
        body: Option<String>,
        content_length: Option<u64>
    ) -> Self {
        let server = format!("{} ({})", ARCH, OS);
        let content_type = String::from("text/html; charset=UTF-8");
        let date = Utc::now();
        
        Response { 
            status_code, 
            body, 
            server,
            content_type,
            content_length,
            date
        }
    }

    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => ""
        };

        write!(
            stream, 
            "HTTP/1.1 {} {} \r\n\r\n{}", 
            self.status_code,
            self.status_code.reason_phrase(),
            body
        )
    }
}
 