pub mod error;
pub mod token;
mod shunting_yard;

use error::Result;
use expression::error::ParseError;
use expression::error::ParseResult;
use expression::token::Token;
use expression::shunting_yard::shunting_yard;


#[derive(Debug)]
pub enum Expression {
//    Number(i64),
//    Add(Box<Expression>, Box<Expression>),
//    Sub(Box<Expression>, Box<Expression>),
//    Mult(Box<Expression>, Box<Expression>),
//    Div(Box<Expression>, Box<Expression>),
//    Mod(Box<Expression>, Box<Expression>),
    Dummy(String),
}


pub fn parse_expr(str: &str) -> Result<Expression> {

    let tokens = token::parse_token(str)?;
    let tokens = shunting_yard(tokens);

    println!("Tokens: {:?}", tokens);

    Ok(Expression::Dummy(str.to_string()))
}





#[cfg(test)]
mod test {
    use super::*;

    //// parse_expr ////
    
} 
