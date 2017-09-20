pub mod error;

use expression::Expression;
use execute::error::ExecuteError;
use execute::error::ExecuteResult;

pub fn execute(expr: Expression) -> ExecuteResult<i64> {
    match expr {
        Expression::Number(n) => Ok(n),
        Expression::Add(left, right) => Ok(execute(*left)? + execute(*right)?),
        _ => unimplemented!(),
    }
}
