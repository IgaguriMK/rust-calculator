use std::error;
use std::io;
use std::fmt;
use std::result;

#[derive(Debug)]
pub enum CalcError {
    Io(io::Error),
}

pub type Result<T> = result::Result<T, CalcError>;

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CalcError::Io(ref err) => write!(f, "IO error:{}", err),
        }
    }
}

impl error::Error for CalcError {
    fn description(&self) -> &str {
        match *self {
            CalcError::Io(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            CalcError::Io(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for CalcError {
    fn from(err: io::Error) -> CalcError {
        CalcError::Io(err)
    }
}

