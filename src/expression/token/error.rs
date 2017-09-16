use std::error;
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
        s = s + &format!("トークン化できない入力が検出されました。\n");
        s = s + &format!("入力: \"{}\"\n", source);
        s = s + &format!("       {}^ 不正な文字\n", filler);
        TokenError::InvalidChar(s)
    }

    pub fn get_message(&self) -> &str {
        match *self {
            TokenError::InvalidChar(ref message) => message,
        }
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
