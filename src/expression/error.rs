use std::error;
use std::fmt;
use std::result;

#[derive(Debug)]
pub enum ParseError {
    MismatchParen(String),
    NoToken(String),
    TooMuchToken(String),
}

impl ParseError {
    pub  fn new_mismatch_paren(message: &str) -> ParseError {
        ParseError::MismatchParen(String::from(message))
    }

    pub fn new_no_token_error(message: &str) -> ParseError {
        ParseError::NoToken(String::from(message))
    }

    pub fn new_too_much_token_error(message: &str) -> ParseError {
        ParseError::TooMuchToken(String::from(message))
    }

    pub fn get_message(&self) -> &str {
        match *self {
            ParseError::MismatchParen(ref message) => message,
            ParseError::NoToken(ref message) => message,
            ParseError::TooMuchToken(ref message) => message,
        }
    }
}

pub type ParseResult<T> = result::Result<T, ParseError>;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError::MismatchParen(ref message) => write!(f, "{}", message),
            ParseError::NoToken(ref message) => write!(f, "{}", message),
            ParseError::TooMuchToken(ref message) => write!(f, "{}", message),
        }
    }
}


impl error::Error for ParseError {
    fn description(&self) -> &str {
        match *self {
            ParseError::MismatchParen(_) => "Paren mismatch.",
            ParseError::NoToken(_) => "Tokens end while parse",
            ParseError::TooMuchToken(_) => "Too much token",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}
