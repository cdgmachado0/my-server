use chrono::prelude::{DateTime, Utc};
use std::env::consts::{OS, ARCH};

pub struct HeadersResp {
    server: String,
    content_type: String, 
    content_length: u64,
    date: DateTime<Utc>
}


impl HeadersResp {
    fn set(length: Option<u64>) -> Self {
        let server = format!("{} ({})", ARCH, OS);
        let content_type = String::from("text/html; charset=UTF-8");
        let date = Utc::now();

        HeadersResp {
            server,
            content_type,
            content_length: length.unwrap(),
            date,
        }
    } //put this impl in response.rs

}