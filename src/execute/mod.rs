pub mod error;

use expression::Expression;
use execute::error::ExecuteError;
use execute::error::ExecuteResult;

pub fn execute(expr: Expression) -> ExecuteResult<i64> {
    match expr {
        Expression::Number(n) => Ok(n),
        Expression::Add(left, right) => Ok(execute(*left)? + execute(*right)?),
        Expression::Sub(left, right) => Ok(execute(*left)? - execute(*right)?),
        Expression::Mult(left, right) => Ok(execute(*left)? * execute(*right)?),
        Expression::Div(left, right) => {
            let right_result = execute(*right)?;
            if right_result != 0 {
                Ok(execute(*left)? / right_result)
            } else {
                Err(ExecuteError::out_of_def("ゼロ除算エラー"))
            }
        },
        Expression::Mod(left, right) => {
            let right_result = execute(*right)?;
            if right_result != 0 {
                Ok(execute(*left)? % right_result)
            } else {
                Err(ExecuteError::out_of_def("ゼロ除算エラー"))
            }
        },
        Expression::Pow(left, right) => {
            let right_result = execute(*right)?;
            if right_result >= 0 {
                Ok(execute(*left)?.pow(right_result as u32))
            } else {
                Err(ExecuteError::out_of_def("指数が負です"))
            }
        },
    }
}
