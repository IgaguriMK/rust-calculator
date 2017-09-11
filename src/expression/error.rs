use std::error;
use std::fmt;
use std::result;

#[derive(Debug)]
pub enum ParseError {
    MismatchParen(String),
    Other(String),
}

impl ParseError {
    pub  fn new_mismatch_paren(message: &str) -> ParseError {
        ParseError::MismatchParen(String::from(message))
    }

    pub fn new_other_error(message: &str) -> ParseError {
        ParseError::Other(String::from(message))
    }

    pub fn get_message(&self) -> &str {
        match *self {
            ParseError::MismatchParen(ref message) => message,
            ParseError::Other(ref message) => message,
        }
    }
}

pub type ParseResult<T> = result::Result<T, ParseError>;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError::MismatchParen(ref message) => write!(f, "{}", message),
            ParseError::Other(ref message) => write!(f, "{}", message),
        }
    }
}


impl error::Error for ParseError {
    fn description(&self) -> &str {
        match *self {
            ParseError::MismatchParen(_) => "Paren mismatch.",
            ParseError::Other(_) => "Unknown error.",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}
