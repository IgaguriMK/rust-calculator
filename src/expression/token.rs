use regex::Regex;
use std::vec::Vec;

#[derive(Debug)]
pub enum Token {
    Number(i64),
    Plus,
}

lazy_static! {
    static ref REG_NUMBER: Regex = {
        Regex::new(r"^[0-9]+").unwrap()
    };

    static ref REG_PLUS: Regex = {
        Regex::new(r"^\+").unwrap()
    };

    static ref REG_SPACE: Regex = {
        Regex::new(r"^[ \t]+").unwrap()
    };
}

pub fn parse_token(str: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut str_left = str;
    let mut pos = 0;

    loop {
        str_left = str_left.trim();

        if str_left == "" {
            return tokens;
        }

        if let Some(mat) = REG_NUMBER.find(str_left) {
            let mat_str = mat.as_str();
            pos += mat_str.len();

            let val = mat_str.parse::<i64>().unwrap();
            tokens.push(Token::Number(val));

            str_left = str_tail_at(str_left, mat.end());

        } else if let Some(mat) = REG_PLUS.find(str_left) {
            let mat_str = mat.as_str();
            pos += mat_str.len();

            tokens.push(Token::Plus);

            str_left = str_tail_at(str_left, mat.end());
            
        } else if let Some(mat) = REG_SPACE.find(str_left) {
            let mat_str = mat.as_str();
            pos += mat_str.len();

            str_left = str_tail_at(str_left, mat.end());

        } else {
            let filler = String::from_utf8(vec![b' '; pos]).unwrap();

            println!("\nParse error occer!");
            println!("Input: \"{}\"", str);
            println!("        {}^ Invalid token here", filler);
            panic!("Invalid token");
        }
    }
}

fn str_tail_at(str: &str, at: usize) -> &str {
    if at < str.len() {
        &str[at..]
    } else {
        ""
    }
}

