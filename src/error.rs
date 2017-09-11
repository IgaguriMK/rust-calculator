use std::error;
use std::io;
use std::fmt;
use std::result;

use expression::token::error::TokenError;

#[derive(Debug)]
pub enum CalcError {
    Io(io::Error),
    Token(TokenError),
}

pub type Result<T> = result::Result<T, CalcError>;

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CalcError::Io(ref err) => write!(f, "IO error:{}", err),
            CalcError::Token(ref err) => write!(f, "{}", err),
        }
    }
}

impl error::Error for CalcError {
    fn description(&self) -> &str {
        match *self {
            CalcError::Io(ref err) => err.description(),
            CalcError::Token(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            CalcError::Io(ref err) => Some(err),
            CalcError::Token(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for CalcError {
    fn from(err: io::Error) -> CalcError {
        CalcError::Io(err)
    }
}

impl From<TokenError> for CalcError {
    fn from(err: TokenError) -> CalcError {
        CalcError::Token(err)
    }
}
