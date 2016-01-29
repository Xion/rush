//! Module implementing an AST node for binary operations.

use std::fmt;
use std::iter;

use eval::{self, Eval, Context, Value};


/// Represents an operation involving binary operators and their arguments.
///
/// Because of the way the operations are parsed, arbitrary length chains
/// of operations with the same priority (e.g. + and -) are represented
/// as one object.
///
pub struct BinaryOpNode {
    pub first: Box<Eval>,
    pub rest: Vec<(String, Box<Eval>)>,
}


impl fmt::Debug for BinaryOpNode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "<Op: {:?} {}>", self.first,
               self.rest.iter()
                   .map(|&(ref op, ref arg)| format!("{} {:?}", op, arg))
                   .collect::<Vec<String>>().join(" "))
    }
}


impl Eval for BinaryOpNode {
    fn eval(&self, context: &Context) -> eval::Result {
        let mut result = try!(self.first.eval(&context));
        for &(ref op, ref arg) in &self.rest {
            let arg = try!(arg.eval(&context));
            match &op[..] {
                "+" => result = try!(BinaryOpNode::eval_plus(result, arg)),
                "-" => result = try!(BinaryOpNode::eval_minus(result, arg)),
                "*" => result = try!(BinaryOpNode::eval_times(result, arg)),
                "/" => result = try!(BinaryOpNode::eval_by(result, arg)),
                "%" => result = try!(BinaryOpNode::eval_modulo(result, arg)),
                _ => { return Err(
                    eval::Error::new(&format!("unknown binary operator: `{}`", op))
                ); }
            }
        }
        Ok(result)
    }
}


impl BinaryOpNode {
    /// Evaluate the "+" operator for two values.
    fn eval_plus(left: Value, right: Value) -> eval::Result {
        eval2!(left, right : &String { left.clone() + &*right });
        eval2!(left, right : Integer { left + right });
        eval2!(left, right : Float { left + right });
        eval2!((left: Integer, right: Float) -> Float { left as f64 + right });
        eval2!((left: Float, right: Integer) -> Float { left + right as f64 });
        BinaryOpNode::err("+", left, right)
    }

    /// Evaluate the "-" operator for two values.
    fn eval_minus(left: Value, right: Value) -> eval::Result {
        eval2!(left, right : Integer { left - right });
        eval2!(left, right : Float { left - right });
        eval2!((left: Integer, right: Float) -> Float { left as f64 - right });
        eval2!((left: Float, right: Integer) -> Float { left - right as f64 });
        BinaryOpNode::err("-", left, right)
    }

    /// Evaluate the "*" operator for two values.
    fn eval_times(left: Value, right: Value) -> eval::Result {
        eval2!(left, right : Integer { left * right });
        eval2!(left, right : Float { left * right });
        eval2!((left: &String, right: Integer) -> String where (right > 0) {
            iter::repeat(left).map(String::clone).take(right as usize).collect()
        });
        BinaryOpNode::err("*", left, right)
    }

    /// Evaluate the "/" operator for two values.
    fn eval_by(left: Value, right: Value) -> eval::Result {
        eval2!(left, right : Integer { left / right });
        eval2!(left, right : Float { left / right });
        BinaryOpNode::err("/", left, right)
    }

    /// Evaluate the "%" operator for two values.
    fn eval_modulo(left: Value, right: Value) -> eval::Result {
        // modulo/remainder
        eval2!(left, right : Integer { left % right });
        eval2!(left, right : Float { left % right });
        eval2!((left: Integer, right: Float) -> Float {
            (left as f64) % right
        });
        eval2!((left: Float, right: Integer) -> Float {
            left % (right as f64)
        });

        // string formatting (for just one argument)
        // TODO(xion): improve:
        // 1) error out for invalid placeholders (e.g. %d for strings)
        // 2) %% for escaping %
        // 3) numeric formatting options
        // the easiest way is probably call real snprintf() with FFI
        eval2!((left: &String, right: &String) -> String {
            left.replace("%s", right)
        });
        eval2!((left: &String, right: Integer) -> String {
            left.replace("%d", &right.to_string())
        });
        eval2!((left: &String, right: Float) -> String {
            left.replace("%f", &right.to_string())
        });

        BinaryOpNode::err("%", left, right)
    }

    /// Produce an error about invalid arguments for an operator.
    fn err(op: &str, left: Value, right: Value) -> eval::Result {
        Err(eval::Error::new(&format!(
            "invalid arguments for `{}` operator: `{:?}` and `{:?}`",
            op, left, right)))
    }
}
