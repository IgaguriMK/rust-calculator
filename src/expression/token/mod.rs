pub mod error;

use regex::Regex;
use std::vec::Vec;

use expression::token::error::TokenError;
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

    static ref REG_NEG_NUMBER: Regex = {
        Regex::new(r"^\(-[0-9]+\)").unwrap()
    };

    static ref REG_PLUS: Regex = {
        Regex::new(r"^\+").unwrap()
    };

    static ref REG_SPACE: Regex = {
        Regex::new(r"^[ \t]+").unwrap()
    };
}

pub fn parse_token(str: &str) -> TokenResult<Vec<Token>> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut str_left = str;
    let mut pos = 0;

    loop {
        str_left = str_left.trim();

        if str_left == "" {
            return Ok(tokens);
        }

        if let Some(mat) = REG_NUMBER.find(str_left) {
            let mat_str = mat.as_str();
            pos += mat_str.len();

            let val = mat_str.parse::<i64>().unwrap();
            tokens.push(Token::Number(val));

            str_left = str_tail_at(str_left, mat.end());

        }else if let Some(mat) = REG_NEG_NUMBER.find(str_left) {
            let mat_str = mat.as_str();
            pos += mat_str.len();

            let mat_str = &mat_str[1 .. (mat_str.len() - 1)];

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
            return Err(
                    TokenError::new_invalid_char(pos, str)
                );
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
        
        let tokens = tokens.expect("Test returns Err().");
        assert_eq!(tokens, vec![Token::Number(1)]);
    }

    #[test]
    fn parse_token_number_neg_1() {
        let tokens = parse_token("(-1)");
        
        let tokens = tokens.expect("Test returns Err().");
        assert_eq!(tokens, vec![Token::Number(-1)]);
    }

    #[test]
    fn parse_token_number_max() {
        let tokens = parse_token("9223372036854775807");
        
        let tokens = tokens.expect("Test returns Err().");
        assert_eq!(tokens, vec![Token::Number(9223372036854775807)]);
    }

    #[test]
    fn parse_token_number_min() {
        let tokens = parse_token("(-9223372036854775808)");
        
        let tokens = tokens.expect("Test returns Err().");
        assert_eq!(tokens, vec![Token::Number(-9223372036854775808)]);
    }

    #[test]
    fn parse_token_add() {
        let tokens = parse_token("+");
        
        let tokens = tokens.expect("Test returns Err().");
        assert_eq!(tokens, vec![Token::Plus]);
    }

    #[test]
    fn parse_token_invalid() {
        let result = parse_token("?");
        
        let err = result.expect_err("This test should be return error.");

        match err {
            TokenError::InvalidChar(_) => (),
            // e => panic!("Unexpected error in test:\n{:?}", e),
        }
    }

    #[test]
    fn parse_token_add_expr() {
        let tokens = parse_token("1+2");
        
        let tokens = tokens.expect("Test returns Err().");
        assert_eq!(tokens, vec![Token::Number(1), Token::Plus, Token::Number(2)]);
    }

    #[test]
    fn parse_token_with_spaces() {
        let tokens = parse_token(" 1  + 2 ");
        
        let tokens = tokens.expect("Test returns Err().");
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
