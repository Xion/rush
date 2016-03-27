//! Module implementing evaluation of unary operator AST nodes.

use eval::{self, api, Eval, Context, Value};
use parse::ast::UnaryOpNode;


impl Eval for UnaryOpNode {
    fn eval(&self, context: &Context) -> eval::Result {
        let arg = try!(self.arg.eval(&context));
        match &self.op[..] {
            "+" => UnaryOpNode::eval_plus(arg),
            "-" => UnaryOpNode::eval_minus(arg),
            "!" => UnaryOpNode::eval_bang(arg),
            _ => Err(eval::Error::new(
                &format!("unknown unary operator: `{}`", self.op)
            ))
        }
    }
}

impl UnaryOpNode {
    /// Evaluate the "+" operator for one value.
    fn eval_plus(arg: Value) -> eval::Result {
        eval1!(arg : Integer { arg });
        eval1!(arg : Float { arg });
        UnaryOpNode::err("+", &arg)
    }

    /// Evaluate the "-" operator for one value.
    fn eval_minus(arg: Value) -> eval::Result {
        eval1!(arg : Integer { -arg });
        eval1!(arg : Float { -arg });
        UnaryOpNode::err("-", &arg)
    }

    /// Evaluate the "!" operator for one value.
    fn eval_bang(arg: Value) -> eval::Result {
        let arg = try!(api::conv::bool(arg)).unwrap_bool();
        Ok(Value::Boolean(!arg))
    }
}

impl UnaryOpNode {
    /// Produce an error about invalid argument for an operator.
    #[inline(always)]
    fn err(op: &str, arg: &Value) -> eval::Result {
        Err(eval::Error::new(&format!(
            "invalid argument for `{}` operator: `{:?}`", op, arg
        )))
    }
}
