extern crate regex;
#[macro_use]
extern crate lazy_static;

mod error;
mod expression;

use std::io;
use std::io::Write;

use error::CalcError;
use error::Result;

fn main() {
    match calc_expr() {
        Ok(()) => (),
        Err(CalcError::Token(token_err)) => {
            println!("{}", token_err.get_message());
        },
        Err(err) => println!("Internal error: {}", err),
    }
}

fn calc_expr() -> Result<()> {
    print!("Input expr:");
    io::stdout().flush()?;

    let mut line = read_line()?;
    line.pop();

    let expr = expression::parse_expr(&line)?;

    println!("bytes:{:?}", expr);

    Ok(())
}

fn read_line() -> Result<String> {
    let stdin = io::stdin();

    let mut line = String::new();
    stdin.read_line(&mut line)?;

    Ok(line)
}
