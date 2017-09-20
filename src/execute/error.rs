use std::error;
use std::fmt;
use std::result;

#[derive(Debug)]
pub enum ExecuteError {
    OutOfDef(String)
}

impl ExecuteError {
    pub fn out_of_def<S>(message: S) -> ExecuteError
    where
        S: Into<String>,
    {
        ExecuteError::OutOfDef(message.into())
    }

    pub fn get_message(&self) -> &str {
        match *self {
            ExecuteError::OutOfDef(ref message) => message,
        }
    }
}

pub type ExecuteResult<T> = result::Result<T, ExecuteError>;

impl fmt::Display for ExecuteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ExecuteError::OutOfDef(ref message) => write!(f, "{}", message),
        }
    }
}


impl error::Error for ExecuteError {
    fn description(&self) -> &str {
        match *self {
            ExecuteError::OutOfDef(_) => "Paren mismatch.",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}
