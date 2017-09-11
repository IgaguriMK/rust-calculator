pub mod error;

use regex::Regex;
use std::vec::Vec;

use expression::token::error::TokenResult;

#[derive(Debug, PartialEq)]
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_token_number_1() {
        let tokens = parse_token("1");
        assert_eq!(tokens, vec![Token::Number(1)]);
    }

    #[test]
    fn parse_token_number_max() {
        let tokens = parse_token("9223372036854775807");
        assert_eq!(tokens, vec![Token::Number(9223372036854775807)]);
    }

    #[test]
    fn parse_token_add() {
        let tokens = parse_token("+");
        assert_eq!(tokens, vec![Token::Plus]);
    }

    #[test]
    #[should_panic(expected = "Invalid token")]
    fn parse_token_invalid() {
        parse_token("?");
    }

    #[test]
    fn parse_token_add_expr() {
        let tokens = parse_token("1+2");
        assert_eq!(tokens, vec![Token::Number(1), Token::Plus, Token::Number(2)]);
    }

    #[test]
    fn parse_token_with_spaces() {
        let tokens = parse_token(" 1  + 2 ");
        assert_eq!(tokens, vec![Token::Number(1), Token::Plus, Token::Number(2)]);
    }


    #[test]
    fn str_tail_at_0() {
        let cut = str_tail_at("abc", 0);
        assert_eq!(cut, "abc");
    }

    #[test]
    fn str_tail_at_1() {
        let cut = str_tail_at("abc", 1);
        assert_eq!(cut, "bc");
    }

    #[test]
    fn str_tail_at_tail() {
        let cut = str_tail_at("abc", 3);
        assert_eq!(cut, "");
    }

    #[test]
    fn str_tail_at_over_tail() {
        let cut = str_tail_at("abc", 4);
        assert_eq!(cut, "");
    }
}
