//! Data structures representing the abstract syntax tree (AST)
//! of parsed expressions.

mod unaryop;

pub use self::unaryop::*;


use std::iter;
use std::str::FromStr;

use eval::{self, Eval, EvalResult, Context, Value};


pub struct AtomNode {
    pub value: Value,
}

impl FromStr for AtomNode {
    type Err = <Value as FromStr>::Err;

    fn from_str(s: &str) -> Result<AtomNode, Self::Err> {
        s.parse::<Value>().map(|v| AtomNode{value: v})
    }
}

impl Eval for AtomNode {
    fn eval(&self, context: &Context) -> EvalResult {
        Ok(self.resolve(&context))
    }
}

impl AtomNode {
    /// Resolve a possible variable reference against given context.
    ///
    /// Returns the variable's Value (which may be just variable name as string),
    /// or a copy of the original Value if it wasn't a reference.
    fn resolve(&self, context: &Context) -> Value {
        let mut result = self.value.clone();

        // follow the chain of references until it bottoms out
        loop {
            match result {
                Value::Symbol(sym) => {
                    result = context.get_var(&sym)
                        .map(Value::clone)
                        .unwrap_or_else(move || Value::String(sym));
                }
                _ => { break; }
            }
        }
        result
    }
}


pub struct BinaryOpNode {
    pub first: Box<Eval>,
    pub rest: Vec<(String, Box<Eval>)>,
}

impl Eval for BinaryOpNode {
    fn eval(&self, context: &Context) -> EvalResult {
        let mut result = try!(self.first.eval(&context));
        for &(ref op, ref arg) in &self.rest {
            let arg = try!(arg.eval(&context));
            match &op[..] {
                "+" => result = try!(BinaryOpNode::eval_plus(&result, &arg)),
                "-" => result = try!(BinaryOpNode::eval_minus(&result, &arg)),
                "*" => result = try!(BinaryOpNode::eval_times(&result, &arg)),
                "/" => result = try!(BinaryOpNode::eval_by(&result, &arg)),
                _ => { return Err(
                    eval::Error::new(&format!("unknown binary operator: `{}`", op))
                ); }
            }
        }
        Ok(result)
    }
}

/// Helper macro for defining how binary operators evaluate
/// for different value types.
///
/// See the usage in BinaryOpNode.eval_X methods below.
macro_rules! binary_op_eval {
    // (left: &Foo, right: &Bar) -> Baz where pre() { foo(left, right) }
    (($x:ident: &$t1:ident, $y:ident: &$t2:ident) -> $rt:ident where $pre:expr { $e:expr }) => {
        if let &Value::$t1(ref $x) = $x {
            if let &Value::$t2(ref $y) = $y {
                if $pre {
                    return Ok(Value::$rt($e));
                }
            }
        }
    };
    // (left: &Foo, right: &Bar) -> Baz { foo(left, right) }
    (($x:ident: &$t1:ident, $y:ident: &$t2:ident) -> $rt:ident { $e:expr }) => {
        binary_op_eval!(($x: &$t1, $y: &$t2) -> $rt where true { $e });
    };

    // (left: &Foo, right: Bar) -> Baz where pre() { foo(left, right) }
    (($x:ident: &$t1:ident, $y:ident: $t2:ident) -> $rt:ident where $pre:expr { $e:expr }) => {
        if let &Value::$t1(ref $x) = $x {
            if let Value::$t2($y) = *$y {
                if $pre {
                    return Ok(Value::$rt($e));
                }
            }
        }
    };
    // (left: &Foo, right: Bar) -> Baz { foo(left, right) }
    (($x:ident: &$t1:ident, $y:ident: $t2:ident) -> $rt:ident { $e:expr }) => {
        binary_op_eval!(($x: &$t1, $y: $t2) -> $rt where true { $e });
    };

    // (left: Foo, right: &Bar) -> Baz where pre() { foo(left, right) }
    (($x:ident: $t1:ident, $y:ident: &$t2:ident) -> $rt:ident where $pre:expr { $e:expr }) => {
        if let Value::$t1($x) = *$x {
            if let &Value::$t2(ref $y) = $y {
                if $pre {
                    return Ok(Value::$rt($e));
                }
            }
        }
    };
    // (left: Foo, right: &Bar)-> Baz { foo(left, right) }
    (($x:ident: $t1:ident, $y:ident: &$t2:ident) -> $rt:ident { $e:expr }) => {
        binary_op_eval!(($x: $t1, $y: &$t2) -> $rt where true { $e });
    };

    // (left: Foo, right: Bar) -> Baz where pre() { foo(left, right) }
    (($x:ident: $t1:ident, $y:ident: $t2:ident) -> $rt:ident where $pre:expr { $e:expr }) => {
        if let Value::$t1($x) = *$x {
            if let Value::$t2($y) = *$y {
                if $pre {
                    return Ok(Value::$rt($e));
                }
            }
        }
    };
    // (left: Foo, right: Bar) -> Baz { foo(left, right) }
    (($x:ident: $t1:ident, $y:ident: $t2:ident) -> $rt:ident { $e:expr }) => {
        binary_op_eval!(($x: $t1, $y: $t2) -> $rt where true { $e });
    };

    // left, right : &Foo { foo(left, right) }
    ($x:ident, $y:ident : &$t:ident { $e:expr }) => {
        binary_op_eval!(($x: &$t, $y: &$t) -> $t where true { $e });
    };
    // left, right : Foo { foo(left, right) }
    ($x:ident, $y:ident : $t:ident { $e:expr }) => {
        binary_op_eval!(($x: $t, $y: $t) -> $t where true { $e });
    };
}
impl BinaryOpNode {
    /// Evaluate the "+" operator for two values.
    fn eval_plus(left: &Value, right: &Value) -> EvalResult {
        binary_op_eval!(left, right : &String { left.clone() + &*right });
        binary_op_eval!(left, right : Integer { left + right });
        binary_op_eval!(left, right : Float { left + right });
        BinaryOpNode::err("+", &left, &right)
    }

    /// Evaluate the "-" operator for two values.
    fn eval_minus(left: &Value, right: &Value) -> EvalResult {
        binary_op_eval!(left, right : Integer { left - right });
        binary_op_eval!(left, right : Float { left - right });
        BinaryOpNode::err("-", &left, &right)
    }

    /// Evaluate the "*" operator for two values.
    fn eval_times(left: &Value, right: &Value) -> EvalResult {
        binary_op_eval!(left, right : Integer { left * right });
        binary_op_eval!(left, right : Float { left * right });
        binary_op_eval!((left: &String, right: Integer) -> String where right > 0 {
            iter::repeat(left).map(String::clone).take(right as usize).collect()
        });
        BinaryOpNode::err("*", &left, &right)
    }

    /// Evaluate the "/" operator for two values.
    fn eval_by(left: &Value, right: &Value) -> EvalResult {
        binary_op_eval!(left, right : Integer { left / right });
        binary_op_eval!(left, right : Float { left / right });
        BinaryOpNode::err("/", &left, &right)
    }

    /// Produce an error about invalid arguments for an operator.
    fn err(op: &str, left: &Value, right: &Value) -> EvalResult {
        Err(eval::Error::new(&format!(
            "invalid arguments for `{}` operator: `{:?}` and `{:?}`",
            op, left, right)))
    }
}


pub struct FunctionCallNode {
    pub name: String,
    pub args: Vec<Box<Eval>>,
}

impl Eval for FunctionCallNode {
    fn eval(&self, context: &Context) -> Result<Value, eval::Error> {
        // evaluate all the arguments first, bail if any of that fails
        let evals: Vec<_> =
            self.args.iter().map(|x| x.eval(&context)).collect();
        if let Some(res) = evals.iter().find(|r| r.is_err()) {
            return res.clone();
        }

        // extract the argument values and call the function
        let args = evals.iter().map(|r| r.clone().ok().unwrap()).collect();
        context.call_func(&self.name, args).ok_or(
            eval::Error::new(&format!("unknown function: {}", self.name)))
    }
}
