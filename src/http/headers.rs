use chrono::prelude::{DateTime, Utc};
use std::env::consts::{OS, ARCH};
use std::collections::HashMap;

#[derive(Debug)]
pub struct HeadersResp {
    server: String,
    content_type: String, 
    content_length: u64,
    date: DateTime<Utc>
}

#[derive(Debug)]
pub struct HeadersReq<'hr> {
    data: Option<HashMap<&'hr str, &'hr str>>
}


impl<'hr> HeadersReq<'hr> {
    pub fn new() -> Self {
       Self { data: None }
    }

    pub fn insert(&mut self, key: &'hr str, value: &'hr str) {
        self.data.get_or_insert_with(HashMap::new).insert(key, value);
    }

    pub fn data(&self) -> Option<&HashMap<&str, &str>> {
        self.data.as_ref()
    }
}

impl HeadersResp {
    pub fn new(content_length: Option<u64>, file_type: ContentType) -> Self {
        let server = format!("{} ({})", ARCH, OS);
        let content_type = match file_type {
            ContentType::HTML | ContentType::NoType => String::from("text/html; charset=UTF-8"),
            ContentType::CSS => String::from("text/css"),
        };
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

#[derive(Debug)]
pub enum ContentType {
    HTML,
    CSS,
    NoType
}