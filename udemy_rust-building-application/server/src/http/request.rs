use crate::http::request;

use super::method::{Method, MethodError};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::Formatter;
use std::fmt::{Debug, Display, Result as FmtResult};
use std::str;
use std::str::Utf8Error;

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1\r\n .. HEADERS...
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buf)?;
        /* Same meaning with upper codes
        case1)
        match str::from_utf8(buf) {
            Ok(request) => {}
            Err(_) => return Err(ParseError::InvalidEncoding),
        }
        case2)
        match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)) {
            Ok(request) => {}
            Err(e) => return Err(e),
        }
        */

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        /* Same meaning with upper codes
        case1)
            match get_next_word(request) {
                Some((method, request)) => {}
                None => return Err(ParseError::InvalidRequest),
            }
        */

        // request variable was shadowed with other type
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;

        if let Some(i) = path.find('?') {
            query_string = Some(&path[i + 1..]);
            path = &path[..i];
        }
        /* Same meaning with upper codes
        case1)
            match path.find('?') {
                Some(i) => {
                query_string = Some(&path[i + 1..]);
                path = &path[..i];
                }
                None => {}
            }
        case2)
            let q = path.find('?');
            if q.is_some() {
                // if option is None, it's getting to panic
                let i = q.unwrap();
                query_string = Some(&path[i + 1..]);
                path = &path[..i];
            }
        */

        unimplemented!()
    }
}

// return nothing by using Option
fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (idx, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..idx], &request[idx + 1..]));
        }
    }

    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn messgae(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.messgae())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.messgae())
    }
}

impl Error for ParseError {}
