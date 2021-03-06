use expression::error::ParseError;
use expression::error::ParseResult;
use expression::token::Associativity;
use expression::token::Token;


pub fn shunting_yard(mut tokens: Vec<Token>) -> ParseResult<Vec<Token>> {
    tokens.reverse();

    let size = tokens.len();
    let mut stack = Vec::<Token>::with_capacity(size);
    let mut output = Vec::<Token>::with_capacity(size);

    loop {
        if let Some(token) = tokens.pop() {
            match token {
                t @ Token::Number(_) => output.push(t),
                t @ Token::Plus => pop_ops(t, &mut output, &mut stack)?,
                t @ Token::Hyphen => pop_ops(t, &mut output, &mut stack)?,
                t @ Token::Asterisk => pop_ops(t, &mut output, &mut stack)?,
                t @ Token::Slash => pop_ops(t, &mut output, &mut stack)?,
                t @ Token::Percent => pop_ops(t, &mut output, &mut stack)?,
                t @ Token::Hat => pop_ops(t, &mut output, &mut stack)?,
                t @ Token::OpenParen => stack.push(t),
                Token::CloseParen => {
                    loop {
                        match stack.pop() {
                            Some(Token::OpenParen) => break,
                            Some(t) => output.push(t),
                            None => return Err(ParseError::mismatch_paren("57")),
                        }
                    }
                }
            }
        } else {
            match stack.pop() {
                None => return Ok(output),
                Some(Token::OpenParen) => return Err(ParseError::mismatch_paren("66")),
                Some(t) => output.push(t),
            }
        }
    }
}


fn pop_ops(
    current_token: Token,
    output: &mut Vec<Token>,
    stack: &mut Vec<Token>,
) -> ParseResult<()> {
    let is_left = current_token.associativity().unwrap() == Associativity::Left;
    let priority = current_token.priority().unwrap();

    loop {
        if let Some(top) = stack.pop() {
            if let Some(top_priority) = top.priority() {
                if is_left && priority <= top_priority || priority < top_priority {

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

    //// shunting_yard ////

    #[test]
    fn shunting_yard_one_number() {
        let result = shunting_yard(vec![Token::Number(1)]);

        let tokens = result.expect("Test returns Err().");
        assert_eq!(tokens, vec![Token::Number(1)]);
    }

    #[test]
    fn shunting_yard_add() {
        let result = shunting_yard(vec![Token::Number(1), Token::Plus, Token::Number(2)]);

        let tokens = result.expect("Test returns Err().");
        assert_eq!(
            tokens,
            vec![Token::Number(1), Token::Number(2), Token::Plus]
        );
    }

    #[test]
    fn shunting_yard_add_mul_pow() {
        let result = shunting_yard(vec![
            Token::Number(1),
            Token::Plus,
            Token::Number(2),
            Token::Asterisk,
            Token::Number(3),
            Token::Hat,
            Token::Number(4),
        ]);

        let tokens = result.expect("Test returns Err().");
        assert_eq!(
            tokens,
            vec![
                Token::Number(1),
                Token::Number(2),
                Token::Number(3),
                Token::Number(4),
                Token::Hat,
                Token::Asterisk,
                Token::Plus,
            ]
        );
    }

    #[test]
    fn shunting_yard_sub_div_mod() {
        let result = shunting_yard(vec![
            Token::Number(1),
            Token::Hyphen,
            Token::Number(2),
            Token::Slash,
            Token::Number(3),
            Token::Percent,
            Token::Number(4),
        ]);

        let tokens = result.expect("Test returns Err().");
        assert_eq!(
            tokens,
            vec![
                Token::Number(1),
                Token::Number(2),
                Token::Number(3),
                Token::Slash,
                Token::Number(4),
                Token::Percent,
                Token::Hyphen,
            ]
        );
    }

    #[test]
    fn shunting_yard_paren() {
        let result = shunting_yard(vec![
            Token::Number(1),
            Token::Asterisk,
            Token::OpenParen,
            Token::Number(2),
            Token::Plus,
            Token::Number(3),
            Token::CloseParen,
        ]);

        let tokens = result.expect("Test returns Err().");
        assert_eq!(
            tokens,
            vec![
                Token::Number(1),
                Token::Number(2),
                Token::Number(3),
                Token::Plus,
                Token::Asterisk,
            ]
        );
    }
    //
    //    #[test]
    //    fn shunting_yard_() {
    //        let result = shunting_yard(
    //                vec![
    //                    Token::Number(1),
    //                ]);
    //
    //        let tokens = result.expect("Test returns Err().");
    //        assert_eq!(tokens,
    //                vec![
    //                    Token::Number(1),
    //                ]);
    //    }
}
