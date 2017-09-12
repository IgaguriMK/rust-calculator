pub mod token;
pub mod error;

use error::Result;
use expression::error::ParseError;
use expression::error::ParseResult;
use expression::token::Associativity;
use expression::token::Token;
use expression::token::TokenType;


#[derive(Debug)]
pub enum Expression {
    Number(i64),
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mult(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    Mod(Box<Expression>, Box<Expression>),
    Dummy(String),
}


pub fn parse_expr(str: &str) -> Result<Expression> {

    let tokens = token::parse_token(str)?;
    let tokens = shunting_yard(tokens);

    println!("Tokens: {:?}", tokens);

    Ok(Expression::Dummy(str.to_string()))
}


pub fn shunting_yard(mut tokens: Vec<Token>) -> ParseResult<Vec<Token>> {
    tokens.reverse();

    let size = tokens.len();
    let mut stack = Vec::<Token>::with_capacity(size);
    let mut output = Vec::<Token>::with_capacity(size);

    loop {
        println!("");
        println!("Tokens: {:?}", tokens);
        println!("Stack:  {:?}", stack);
        println!("Output: {:?}", output);

        if let Some(token) = tokens.pop() {
            match token {
                t @ Token::Number(_) => output.push(t),
                t @ Token::Plus     => pop_ops(t, &mut output, &mut stack)?,
                t @ Token::Hyphen   => pop_ops(t, &mut output, &mut stack)?,
                t @ Token::Asterisk => pop_ops(t, &mut output, &mut stack)?,
                t @ Token::Slash    => pop_ops(t, &mut output, &mut stack)?,
                t @ Token::Percent  => pop_ops(t, &mut output, &mut stack)?,
                t @ Token::Hat      => pop_ops(t, &mut output, &mut stack)?,
                t @ Token::OpenParen => stack.push(t),
                Token::CloseParen => {
                        loop {
                            match stack.pop() {
                                Some(Token::OpenParen) => break,
                                Some(t) => output.push(t),
                                None => return Err(ParseError::new_mismatch_paren("57")),
                            }
                        }
                    },
                _ => unimplemented!(),
            }
        } else {
            match stack.pop() {
                None => return Ok(output),
                Some(Token::OpenParen) => return Err(ParseError::new_mismatch_paren("66")),
                Some(t) => output.push(t),
            }
        }
    }

    panic!("Should not reach shunting_yard() end");
}


fn pop_ops(current_token: Token, output: &mut Vec<Token>, stack: &mut Vec<Token>) -> ParseResult<()> {
    let is_left = current_token.associativity().unwrap() == Associativity::Left;
    let priority = current_token.priority().unwrap();

    loop {
        if let Some(top) = stack.pop() { 
            if let Some(top_priority) = top.priority() {
                if is_left && priority <= top_priority
                    || priority < top_priority {

                    output.push(top);
                    continue;
                } else {
                    stack.push(top);
                }
            } else {
                stack.push(top);
            }
        }
        
        stack.push(current_token);
        return Ok(());
    }
}



#[cfg(test)]
mod test {
    use super::*;

    //// parse_expr ////
    
    
    //// shunting_yard ////

    #[test]
    fn shunting_yard_one_number() {
        let result = shunting_yard(
                vec![
                    Token::Number(1),
                ]);

        let tokens = result.expect("Test returns Err().");
        assert_eq!(tokens, 
                vec![
                    Token::Number(1),
                ]);
    }
}
