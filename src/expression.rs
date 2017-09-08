
#[derive(Debug)]
pub enum Expression {
    //Value(i64),
    Add(Box<Expression>, Box<Expression>),
    //Mul(Box<Expression>, Box<Expression>),
    Dummy(String),
}

pub fn parse_add(mut str: String) -> Expression {

    match find_char(&str) {
        Some(i) => {
            let tail = str.split_off(i+1);
            str.pop();
            
            let left = Box::new(Expression::Dummy(str));
            let right = Box::new(parse_add(tail));
            Expression::Add(left, right)
        },
        None =>Expression::Dummy(String::from(str))
    }
}

fn find_char(str: &String) -> Option<usize> {
    for (i, ch) in str.chars().enumerate() {
        if ch == '+' {
            return Option::Some(i);
        }
    }

    Option::None
}
