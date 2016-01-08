//! Module implementing the AST node for unary operation.

use eval::{self, Eval, EvalResult, Context, Value};


/// Represents an operation involving a unary operator and its argument.
pub struct UnaryOpNode {
    pub op: String,
    pub arg: Box<Eval>,
}


impl Eval for UnaryOpNode {
    fn eval(&self, context: &Context) -> EvalResult {
        let arg = try!(self.arg.eval(&context));
        match &self.op[..] {
            "+" => UnaryOpNode::eval_plus(&arg),
            "-" => UnaryOpNode::eval_minus(&arg),
            "!" => UnaryOpNode::eval_bang(&arg),
            _ => Err(eval::Error::new(
                &format!("unknown unary operator: `{}`", self.op)
            ))
        }
    }
}


/// Helper macro for defining how unary operators evaluate
/// for different value types.
///
/// See the usage in UnaryOpNode.eval_X methods below.
macro_rules! eval {
    // (arg: &Foo) -> Bar { foo(arg) }
    (($x:ident: &$t:ident) -> $rt:ident { $e:expr }) => {
        if let &Value::$t(ref $x) = $x {
            return Ok(Value::$rt($e));
        }
    };
    // (arg: Foo) -> Bar { foo(arg) }
    (($x:ident: $t:ident) -> $rt:ident { $e:expr }) => {
        if let Value::$t($x) = *$x {
            return Ok(Value::$rt($e));
        }
    };

    // arg : &Foo { foo(arg) }
    ($x:ident : &$t:ident { $e:expr }) => {
        eval!(($x: &$t) -> $t { $e });
    };
    // arg : Foo { foo(arg) }
    ($x:ident : $t:ident { $e:expr }) => {
        eval!(($x: $t) -> $t { $e });
    };
}

impl UnaryOpNode {
    /// Evaluate the "+" operator for one value.
    fn eval_plus(arg: &Value) -> EvalResult {
        eval!(arg : Integer { arg });
        eval!(arg : Float { arg });
        UnaryOpNode::err("+", &arg)
    }

    /// Evaluate the "-" operator for one value.
    fn eval_minus(arg: &Value) -> EvalResult {
        eval!(arg : Integer { -arg });
        eval!(arg : Float { -arg });
        UnaryOpNode::err("-", &arg)
    }

    /// Evaluate the "!" operator for one value.
    fn eval_bang(arg: &Value) -> EvalResult {
        eval!(arg : Boolean { !arg });
        UnaryOpNode::err("!", &arg)
    }

    /// Produce an error about invalid argument for an operator.
    fn err(op: &str, arg: &Value) -> EvalResult {
        Err(eval::Error::new(&format!(
            "invalid argument for `{}` operator: `{:?}`", op, arg
        )))
    }
}
