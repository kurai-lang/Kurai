use vyn_core::scope::Scope;
use vyn_ast::expr::Expr;
use vyn_types::value::Value;
use vyn_binop::bin_op::BinOp;

pub fn expr_to_value(expr: &Expr, scope: &mut Scope) -> Option<Value> {
    match expr {
        // Expr::Literal(val) => Some(val.clone()),
        // Expr::Var(name) => scope.0.get(name).cloned(),
        Expr::Binary { op, left, right } => {
            let l = expr_to_value(left, scope)?;
            let r = expr_to_value(right, scope)?;

            match (op, l, r) {
                (BinOp::Add, Value::Int(a), Value::Int(b)) => Some(Value::Int(a + b)),
                (BinOp::Sub, Value::Int(a), Value::Int(b)) => Some(Value::Int(a - b)),
                (BinOp::Mul, Value::Int(a), Value::Int(b)) => Some(Value::Int(a * b)),
                (BinOp::Div, Value::Int(a), Value::Int(b)) => Some(Value::Int(a / b)),

                (BinOp::Eq,  a, b) => Some(Value::Bool(a == b)),
                (BinOp::Ne,  a, b) => Some(Value::Bool(a != b)),
                (BinOp::Lt,  Value::Int(a), Value::Int(b)) => Some(Value::Bool(a < b)),
                (BinOp::Le,  Value::Int(a), Value::Int(b)) => Some(Value::Bool(a <= b)),
                (BinOp::Gt,  Value::Int(a), Value::Int(b)) => Some(Value::Bool(a > b)),
                (BinOp::Ge,  Value::Int(a), Value::Int(b)) => Some(Value::Bool(a >= b)),

                _ => None, // unsupported combo
            }
        }

        _ => None, // Var, FnCall, etc. can't be reduced to literal
    }
}
