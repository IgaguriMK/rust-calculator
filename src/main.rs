use std::error;
use std::fmt;
use std::io;
use std::io::Write;
use std::result;

fn main() {
    match calc_expr() {
        Ok(()) => (),
        Err(err) => println!("Error: {}", err),
    }
}

fn calc_expr() -> Result<()> {
    print!("Input expr:");
    io::stdout().flush()?;

    let line = read_expr()?;

    println!("bytes:{}", line);

    Ok(())
}

fn read_expr() -> Result<String> {
    let stdin = io::stdin();
    
    let mut line = String::new();
    stdin.read_line(&mut line)?;

    Ok(line)
}


#[derive(Debug)]
enum CalcError {
    Io(io::Error),
}

type Result<T> = result::Result<T, CalcError>;

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CalcError::Io(ref err) => write!(f, "IO error:{}", err),
        }
    }
}

impl error::Error for CalcError {
    fn description(&self) -> &str {
        match *self {
            CalcError::Io(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            CalcError::Io(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for CalcError {
    fn from(err: io::Error) -> CalcError {
        CalcError::Io(err)
    }
}
