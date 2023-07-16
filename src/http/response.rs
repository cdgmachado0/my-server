use std::io::{Write, Result as IoResult};
use super::StatusCode;
use super::headers::HeadersResp;


#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
    headers: HeadersResp
}


impl Response {
    pub fn new(
        status_code: StatusCode, 
        body: Option<String>,
        content_length: Option<u64>
    ) -> Self {
        Response { 
            status_code, 
            body, 
            headers: HeadersResp::new(content_length)
        }
    }

    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => ""
        };

        write!(
            stream, 
            "HTTP/1.1 {} {}\r\nServer: {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nDate: {} \r\n\r\n{}", 
            self.status_code,
            self.status_code.reason_phrase(),
            self.headers.server(),
            self.headers.content_type(),
            self.headers.content_length(),
            self.headers.date(),
            body
        )
    }
}
 