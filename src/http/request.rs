use super::method::{Method, MethodError};
use std::convert::TryFrom;
use std::error::Error;
use std::str::{FromStr, Utf8Error};
use std::str;
use std::fmt::{Formatter, Display, Result as FmtResult, write, Debug};

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buf)?;

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        println!("Method {}", method);
        println!("Path {}", path);
        println!("Protocol {}", protocol);

        let method : Method = method.parse()?;

        let mut query_string = None;
        // match path.find('?') {
        //     Some(index) => {
        //         query_string = Some(&path[i + 1..]);
        //         path = &path[..i];
        //     },
        //     None => {}
        // }

        // let q = path.find('?');
        // if q.is_some() {
        //     let i = q.unwrap();
        //     query_string = Some(&path[i + 1..]);
        //     path = &path[..i];
        // }

        // paten matching
        if let Some(i) = path.find('?') {
            query_string = Some(&path[i + 1..]);
            path = &path[..i];
        }

        unimplemented!()
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)>  {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i+1..]));
        }
    }
    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod
}

impl ParseError {
    fn message(&self) -> &str{
        let message = match self {
            Self::InvalidRequest => "Invalid request",
            Self::InvalidEncoding => "Invalid encoding",
            Self::InvalidProtocol => "Invalid protocol",
            Self::InvalidMethod => "Invalid method"
        };
        message
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
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}