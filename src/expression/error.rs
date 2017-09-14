use std::error;
use std::fmt;
use std::result;

#[derive(Debug)]
pub enum ParseError {
    MismatchParen(String),
    NoToken(String)
}

impl ParseError {
    pub  fn new_mismatch_paren(message: &str) -> ParseError {
        ParseError::MismatchParen(String::from(message))
    }

    pub fn new_no_token_error(message: &str) -> ParseError {
        ParseError::NoToken(String::from(message))
    }

    pub fn get_message(&self) -> &str {
        match *self {
            ParseError::MismatchParen(ref message) => message,
            ParseError::NoToken(ref message) => message,
        }
    }
}

pub type ParseResult<T> = result::Result<T, ParseError>;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError::MismatchParen(ref message) => write!(f, "{}", message),
            ParseError::NoToken(ref message) => write!(f, "{}", message),
        }
    }
}


impl error::Error for ParseError {
    fn description(&self) -> &str {
        match *self {
            ParseError::MismatchParen(_) => "Paren mismatch.",
            ParseError::NoToken(_) => "Tokens end while parse",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}
