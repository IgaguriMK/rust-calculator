use std::error;
use std::fmt;
use std::result;

#[derive(Debug)]
pub enum ExecuteError {
    ZeroDiv(String)
}

impl ExecuteError {
    pub fn zero_div<S>(message: S) -> ExecuteError
    where
        S: Into<String>,
    {
        ExecuteError::ZeroDiv(message.into())
    }

    pub fn get_message(&self) -> &str {
        match *self {
            ExecuteError::ZeroDiv(ref message) => message,
        }
    }
}

pub type ExecuteResult<T> = result::Result<T, ExecuteError>;

impl fmt::Display for ExecuteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ExecuteError::ZeroDiv(ref message) => write!(f, "{}", message),
        }
    }
}


impl error::Error for ExecuteError {
    fn description(&self) -> &str {
        match *self {
            ExecuteError::ZeroDiv(_) => "Paren mismatch.",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}
