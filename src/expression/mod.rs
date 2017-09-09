mod token;

use error::Result;


#[derive(Debug)]
pub enum Expression {
    //Value(i64),
    //Add(Box<Expression>, Box<Expression>),
    //Mul(Box<Expression>, Box<Expression>),
    Dummy(String),
}





pub fn parse_expr(str: &str) -> Result<Expression> {

    let tokens = token::parse_token(str);

    println!("Tokens: {:?}", tokens);

    Ok(Expression::Dummy(str.to_string()))
}



