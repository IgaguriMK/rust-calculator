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
    pub fn mismatch_paren<S>(message: S) -> ParseError
    where
        S: Into<String>,
    {
        ParseError::MismatchParen(message.into())
    }

    pub fn no_token<S>(message: S) -> ParseError
    where
        S: Into<String>,
    {
        ParseError::NoToken(message.into())
    }

    pub fn too_much_token<S>(message: S) -> ParseError
    where
        S: Into<String>,
    {
        ParseError::TooMuchToken(message.into())
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
