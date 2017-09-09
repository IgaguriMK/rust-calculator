mod error;
mod expression;

use std::io;
use std::io::Write;

use error::Result;

fn main() {
    match calc_expr() {
        Ok(()) => (),
        Err(err) => println!("Error: {}", err),
    }
}

fn calc_expr() -> Result<()> {
    print!("Input expr:");
    io::stdout().flush()?;

    let mut line = read_line()?;
    line.pop();

    let expr = expression::parse_add(line);

    println!("bytes:{:?}", expr);

    Ok(())
}

fn read_line() -> Result<String> {
    let stdin = io::stdin();

    let mut line = String::new();
    stdin.read_line(&mut line)?;

    Ok(line)
}
