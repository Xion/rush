//! Module implementing an AST node for binary operations.

use std::iter;

use eval::{self, Eval, EvalResult, Context, Value};


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
                "%" => result = try!(BinaryOpNode::eval_modulo(&result, &arg)),
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
macro_rules! eval {
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
        eval!(($x: &$t1, $y: &$t2) -> $rt where true { $e });
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
        eval!(($x: &$t1, $y: $t2) -> $rt where true { $e });
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
        eval!(($x: $t1, $y: &$t2) -> $rt where true { $e });
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
        eval!(($x: $t1, $y: $t2) -> $rt where true { $e });
    };

    // left, right : &Foo { foo(left, right) }
    ($x:ident, $y:ident : &$t:ident { $e:expr }) => {
        eval!(($x: &$t, $y: &$t) -> $t where true { $e });
    };
    // left, right : Foo { foo(left, right) }
    ($x:ident, $y:ident : $t:ident { $e:expr }) => {
        eval!(($x: $t, $y: $t) -> $t where true { $e });
    };
}

impl BinaryOpNode {
    /// Evaluate the "+" operator for two values.
    fn eval_plus(left: &Value, right: &Value) -> EvalResult {
        eval!(left, right : &String { left.clone() + &*right });
        eval!(left, right : Integer { left + right });
        eval!(left, right : Float { left + right });
        BinaryOpNode::err("+", &left, &right)
    }

    /// Evaluate the "-" operator for two values.
    fn eval_minus(left: &Value, right: &Value) -> EvalResult {
        eval!(left, right : Integer { left - right });
        eval!(left, right : Float { left - right });
        BinaryOpNode::err("-", &left, &right)
    }

    /// Evaluate the "*" operator for two values.
    fn eval_times(left: &Value, right: &Value) -> EvalResult {
        eval!(left, right : Integer { left * right });
        eval!(left, right : Float { left * right });
        eval!((left: &String, right: Integer) -> String where right > 0 {
            iter::repeat(left).map(String::clone).take(right as usize).collect()
        });
        BinaryOpNode::err("*", &left, &right)
    }

    /// Evaluate the "/" operator for two values.
    fn eval_by(left: &Value, right: &Value) -> EvalResult {
        eval!(left, right : Integer { left / right });
        eval!(left, right : Float { left / right });
        BinaryOpNode::err("/", &left, &right)
    }

    /// Evaluate the "%" operator for two values.
    fn eval_modulo(left: &Value, right: &Value) -> EvalResult {
        // modulo/remainder
        eval!(left, right : Integer { left % right });
        eval!(left, right : Float { left % right });
        eval!((left: Integer, right: Float) -> Float {
            (left as f64) % right
        });
        eval!((left: Float, right: Integer) -> Float {
            left % (right as f64)
        });

        // string formatting (for just one argument)
        // TODO(xion): improve:
        // 1) error out for invalid placeholders (e.g. %d for strings)
        // 2) %% for escaping %
        // 3) numeric formatting options
        // the easiest way is probably call real snprintf() with FFI
        eval!((left: &String, right: &String) -> String {
            left.replace("%s", &right)
        });
        eval!((left: &String, right: Integer) -> String {
            left.replace("%d", &right.to_string())
        });
        eval!((left: &String, right: Float) -> String {
            left.replace("%f", &right.to_string())
        });

        BinaryOpNode::err("%", &left, &right)
    }

    /// Produce an error about invalid arguments for an operator.
    fn err(op: &str, left: &Value, right: &Value) -> EvalResult {
        Err(eval::Error::new(&format!(
            "invalid arguments for `{}` operator: `{:?}` and `{:?}`",
            op, left, right)))
    }
}
