use chrono::prelude::{DateTime, Utc};
use std::env::consts::{OS, ARCH};

#[derive(Debug)]
pub struct HeadersResp {
    server: String,
    content_type: String, 
    content_length: u64,
    date: DateTime<Utc>
}


impl HeadersResp {
    pub fn new(content_length: Option<u64>) -> Self {
        let server = format!("{} ({})", ARCH, OS);
        let content_type = String::from("text/html; charset=UTF-8");
        let date = Utc::now();

        HeadersResp {
            server,
            content_type,
            content_length: content_length.unwrap(),
            date,
        }
    }

    pub fn server(&self) -> &str {
        &self.server
    }

    pub fn content_type(&self) -> &str {
        &self.content_type
    }

    pub fn content_length(&self) -> &u64 {
        &self.content_length
    }

    pub fn date(&self) -> &DateTime<Utc> {
        &self.date
    }
}