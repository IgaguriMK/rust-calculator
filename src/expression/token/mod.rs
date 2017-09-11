pub mod error;

use regex::Regex;
use std::vec::Vec;

use expression::token::error::TokenError;
use expression::token::error::TokenResult;

#[derive(Debug, PartialEq)]
pub enum Token {
    Number(i64),
    Plus,
    Hyphen,
    Asterisk,
    Slash,
    Percent,
    OpenParen,
    CloseParen,
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Number,
    Operator,
    Paren,
}

#[derive(Debug, PartialEq)]
pub enum Associativity {
    Left,
    Right,
    Non,
}

impl Token { 
    pub fn priority(&self) -> Option<u8> {
        match *self {
            Token::Plus => Some(1),
            Token::Hyphen => Some(1),
            Token::Asterisk => Some(3),
            Token::Hyphen => Some(3),
            Token::Percent => Some(2),
            _ => None,
        }
    }

    pub fn token_type(&self) -> TokenType {
        match *self {
            Token::Number(_)  => TokenType::Number,
            Token::Plus       => TokenType::Operator,
            Token::Hyphen     => TokenType::Operator,
            Token::Asterisk   => TokenType::Operator,
            Token::Slash      => TokenType::Operator,
            Token::Percent    => TokenType::Operator,
            Token::OpenParen  => TokenType::Paren,
            Token::CloseParen => TokenType::Paren,
        }
    }

    pub fn associativity(&self) -> Option<Associativity> {
        match *self {
            Token::Number(_)  => None,
            Token::Plus       => Some(Associativity::Left),
            Token::Hyphen     => Some(Associativity::Left),
            Token::Asterisk   => Some(Associativity::Left),
            Token::Slash      => Some(Associativity::Left),
            Token::Percent    => Some(Associativity::Left),
            Token::OpenParen  => None,
            Token::CloseParen => None,
        }
    }
}


lazy_static! {
    static ref REG_SPACE: Regex = {
        Regex::new(r"^[ \t]+").unwrap()
    };

    static ref REG_NUMBER: Regex = {
        Regex::new(r"^[0-9]+").unwrap()
    };

    static ref REG_NEG_NUMBER: Regex = {
        Regex::new(r"^\(-[0-9]+\)").unwrap()
    };

    static ref REG_PLUS: Regex = {
        Regex::new(r"^\+").unwrap()
    };

    static ref REG_SUB: Regex = {
        Regex::new(r"^-").unwrap()
    };

    static ref REG_MULT: Regex = {
        Regex::new(r"^\*").unwrap()
    };

    static ref REG_DIV: Regex = {
        Regex::new(r"^/").unwrap()
    };

    static ref REG_MOD: Regex = {
        Regex::new(r"^%").unwrap()
    };

    static ref REG_OPEN_PAREN: Regex = {
        Regex::new(r"^\(").unwrap()
    };

    static ref REG_CLOSE_PAREN: Regex = {
        Regex::new(r"^\)").unwrap()
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
            
        } else if let Some(mat) = REG_SUB.find(str_left) {
            let mat_str = mat.as_str();
            pos += mat_str.len();

            tokens.push(Token::Hyphen);

            str_left = str_tail_at(str_left, mat.end());
            
        } else if let Some(mat) = REG_MULT.find(str_left) {
            let mat_str = mat.as_str();
            pos += mat_str.len();

            tokens.push(Token::Asterisk);

            str_left = str_tail_at(str_left, mat.end());
            
        } else if let Some(mat) = REG_DIV.find(str_left) {
            let mat_str = mat.as_str();
            pos += mat_str.len();

            tokens.push(Token::Slash);

            str_left = str_tail_at(str_left, mat.end());
            
        } else if let Some(mat) = REG_MOD.find(str_left) {
            let mat_str = mat.as_str();
            pos += mat_str.len();

            tokens.push(Token::Percent);

            str_left = str_tail_at(str_left, mat.end());
            
        } else if let Some(mat) = REG_OPEN_PAREN.find(str_left) {
            let mat_str = mat.as_str();
            pos += mat_str.len();

            tokens.push(Token::OpenParen);

            str_left = str_tail_at(str_left, mat.end());
            
        } else if let Some(mat) = REG_CLOSE_PAREN.find(str_left) {
            let mat_str = mat.as_str();
            pos += mat_str.len();

            tokens.push(Token::CloseParen);

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

    //// parse_token ////

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
    fn parse_token_sub() {
        let tokens = parse_token("-");
        
        let tokens = tokens.expect("Test returns Err().");
        assert_eq!(tokens, vec![Token::Hyphen]);
    }

    #[test]
    fn parse_token_mult() {
        let tokens = parse_token("*");
        
        let tokens = tokens.expect("Test returns Err().");
        assert_eq!(tokens, vec![Token::Asterisk]);
    }

    #[test]
    fn parse_token_div() {
        let tokens = parse_token("/");
        
        let tokens = tokens.expect("Test returns Err().");
        assert_eq!(tokens, vec![Token::Slash]);
    }

    #[test]
    fn parse_token_mod() {
        let tokens = parse_token("%");
        
        let tokens = tokens.expect("Test returns Err().");
        assert_eq!(tokens, vec![Token::Percent]);
    }

    #[test]
    fn parse_token_open_paren() {
        let tokens = parse_token("(");
        
        let tokens = tokens.expect("Test returns Err().");
        assert_eq!(tokens, vec![Token::OpenParen]);
    }

    #[test]
    fn parse_token_close_paren() {
        let tokens = parse_token(")");
        
        let tokens = tokens.expect("Test returns Err().");
        assert_eq!(tokens, vec![Token::CloseParen]);
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
    fn parse_token_complex_1() {
        let tokens = parse_token("(1 + 3) %(-6)");
        
        let tokens = tokens.expect("Test returns Err().");
        assert_eq!(tokens, 
                vec![
                    Token::OpenParen,
                    Token::Number(1),
                    Token::Plus,
                    Token::Number(3),
                    Token::CloseParen,
                    Token::Percent,
                    Token::Number(-6),
                ]);
    }

    //// str_tail ////

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
