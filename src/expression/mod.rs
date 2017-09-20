pub mod error;
pub mod token;
mod shunting_yard;

use error::CalcError;
use error::Result;
use expression::error::ParseError;
use expression::error::ParseResult;
use expression::token::Token;
use expression::shunting_yard::shunting_yard;


#[derive(Debug, PartialEq)]
pub enum Expression {
    Number(i64),
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mult(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    Mod(Box<Expression>, Box<Expression>),
    Pow(Box<Expression>, Box<Expression>),
}

impl Expression {
    fn new_box_number(n: i64) -> Box<Expression> {
        Box::new(Expression::Number(n))
    }
    fn new_box_add(l: Box<Expression>, r: Box<Expression>) -> Box<Expression> {
        Box::new(Expression::Add(l, r))
    }
    fn new_box_sub(l: Box<Expression>, r: Box<Expression>) -> Box<Expression> {
        Box::new(Expression::Sub(l, r))
    }
    fn new_box_mult(l: Box<Expression>, r: Box<Expression>) -> Box<Expression> {
        Box::new(Expression::Mult(l, r))
    }
    fn new_box_div(l: Box<Expression>, r: Box<Expression>) -> Box<Expression> {
        Box::new(Expression::Div(l, r))
    }
    fn new_box_mod(l: Box<Expression>, r: Box<Expression>) -> Box<Expression> {
        Box::new(Expression::Mod(l, r))
    }
    fn new_box_pow(l: Box<Expression>, r: Box<Expression>) -> Box<Expression> {
        Box::new(Expression::Pow(l, r))
    }
}


pub fn parse_expr(str: &str) -> Result<Expression> {

    let tokens = token::parse_token(str)?;
    let mut tokens = shunting_yard(tokens)?;
    let expression = build_expression_tree(&mut tokens)?;

    if !tokens.is_empty() {
        let message = format!("トークンが多すぎます。");
        return Err(CalcError::Parse(ParseError::too_much_token(message)));
    }

    Ok(*expression)
}

fn build_expression_tree(tokens: &mut Vec<Token>) -> ParseResult<Box<Expression>> {
    if let Some(token) = tokens.pop() {

        match token {
            Token::Number(n) => return Ok(Expression::new_box_number(n)),
            Token::Plus => {
                let right = build_expression_tree(tokens)?;
                let left = build_expression_tree(tokens)?;
                return Ok(Expression::new_box_add(left, right));
            }
            Token::Hyphen => {
                let right = build_expression_tree(tokens)?;
                let left = build_expression_tree(tokens)?;
                return Ok(Expression::new_box_sub(left, right));
            }
            Token::Asterisk => {
                let right = build_expression_tree(tokens)?;
                let left = build_expression_tree(tokens)?;
                return Ok(Expression::new_box_mult(left, right));
            }
            Token::Slash => {
                let right = build_expression_tree(tokens)?;
                let left = build_expression_tree(tokens)?;
                return Ok(Expression::new_box_div(left, right));
            }
            Token::Percent => {
                let right = build_expression_tree(tokens)?;
                let left = build_expression_tree(tokens)?;
                return Ok(Expression::new_box_mod(left, right));
            }
            Token::Hat => {
                let right = build_expression_tree(tokens)?;
                let left = build_expression_tree(tokens)?;
                return Ok(Expression::new_box_pow(left, right));
            }
            Token::OpenParen => panic!("build_expression_tree(): invalid token 'OpenParen'"),
            Token::CloseParen => panic!("build_expression_tree(): invalid token 'CloseParen'"),
        }

    } else {
        return Err(ParseError::no_token(
            "式の途中でトークンが無くなりました。",
        ));
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use expression::error::ParseError;

    //// parse_expr ////



    //// build_expression_tree ////

    #[test]
    fn build_expression_tree_add() {
        let mut tokens = vec![Token::Number(1), Token::Number(2), Token::Plus];

        let result = build_expression_tree(&mut tokens);

        let expr = result.expect("Test returns Err().");
        assert_eq!(
            expr,
            Expression::new_box_add(Expression::new_box_number(1), Expression::new_box_number(2))
        );
    }

    #[test]
    fn build_expression_tree_add_mult_pow() {
        let mut tokens = vec![
            Token::Number(1),
            Token::Number(2),
            Token::Plus,
            Token::Number(3),
            Token::Asterisk,
            Token::Number(4),
            Token::Hat,
        ];

        let result = build_expression_tree(&mut tokens);

        let expr = result.expect("Test returns Err().");
        assert_eq!(
            expr,
            Expression::new_box_pow(
                Expression::new_box_mult(
                    Expression::new_box_add(Expression::new_box_number(1), Expression::new_box_number(2)),
                    Expression::new_box_number(3),
                ),
                Expression::new_box_number(4),
            )
        );
    }

    #[test]
    fn build_expression_tree_sub_div_mod() {
        let mut tokens = vec![
            Token::Number(1),
            Token::Number(2),
            Token::Hyphen,
            Token::Number(3),
            Token::Slash,
            Token::Number(4),
            Token::Percent,
        ];

        let result = build_expression_tree(&mut tokens);

        let expr = result.expect("Test returns Err().");
        assert_eq!(
            expr,
            Expression::new_box_mod(
                Expression::new_box_div(
                    Expression::new_box_sub(Expression::new_box_number(1), Expression::new_box_number(2)),
                    Expression::new_box_number(3),
                ),
                Expression::new_box_number(4),
            )
        );
    }

    #[test]
    fn build_expression_tree_fail_no_number() {
        let mut tokens = vec![Token::Number(1), Token::Plus];

        let result = build_expression_tree(&mut tokens);

        let expr = result.expect_err("Test should returns Err().");
        match expr {
            ParseError::NoToken(_) => return,
            e => panic!("Unexcepted error: {:?}", e),
        }
    }
}
