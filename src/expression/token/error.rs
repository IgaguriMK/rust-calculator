use std::error;
use std::io;
use std::fmt;
use std::result;

#[derive(Debug)]
pub enum TokenError {
    InvalidChar(String),
}

impl TokenError {
    pub fn new_invalid_char(at: usize, source: &str) -> TokenError {
        let filler = String::from_utf8(vec![b' '; at]).unwrap();

        let mut s = String::new();
        s = s + &format!("\nParse error occered!\n");
        s = s + &format!("Input: \"{}\"\n", source);
        s = s + &format!("    {}^ Invalid token here\n", filler);
        TokenError::InvalidChar(s)
    }
}

pub type TokenResult<T> = result::Result<T, TokenError>;

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TokenError::InvalidChar(ref message) => write!(f, "{}", message),
        }
    }
}

impl error::Error for TokenError {
    fn description(&self) -> &str {
        match *self {
            TokenError::InvalidChar(_) => "Invalid token found.",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}
