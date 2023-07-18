use super::method::{Method, MethodError};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult, Debug};
use std::str;
use std::str::Utf8Error;
use super::{QueryString, headers::HeadersReq};

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method
} 

impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    fn try_from(buf: &'buf [u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buf)?;

        let req = &request;
        get_next_header(&req);

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol)
        }

        let method: Method = method.parse()?;

        let mut query_string = None;

        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        Ok(Self {
            path,
            query_string,
            method
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> { 
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]))
        }
    }
    None
}

// -> Option<&str>
fn get_next_header(request: &str) { 
    let keys = [
        "Host",
        "User-Agent",
        "Accept",
        "Accept-Language"
    ];

    let headers = HeadersReq::new();
    let borrow_head = &mut headers.data();

    for (i, key) in keys.iter().enumerate() { 
        let index = request.rfind(key).unwrap();
        let last_i = index + key.len() + 2;

        for (i, c) in request.chars().enumerate().skip(last_i) { 
            if c == '\r' && request.chars().nth(i+1).unwrap() == '\n' {
                borrow_head.insert(key, &request[last_i..i - 1]);
                // return Some(&request[last_i..i - 1])
            }
        }

    }

    
    // let host_str = "Host";
    // let index = request.rfind(host_str).unwrap();
    // let last_i = index + host_str.len() + 2;
    
    // for (i, c) in request.chars().enumerate().skip(last_i) { 
    //     if c == '\r' && request.chars().nth(i+1).unwrap() == '\n' {
    //         return Some(&request[last_i..i - 1])
    //     }
    // }
    // None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method"
        }
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}