use crate::parser::Expr;
use crate::lexer::Token;

pub fn evaluate(expr: &Expr) -> i32 {
    match expr {
        Expr::Number(n) => *n,
        Expr::BinaryOp { left, op, right } => {
            let lval = evaluate(left);
            let rval = evaluate(right);
            match op {
                Token::Plus => lval + rval,
                Token::Minus => lval - rval,
                Token::Star => lval * rval,
                Token::Slash => lval / rval,
                _ => panic!("Invalid binary operator: {:?}", op),
            }
        }
    }
}
