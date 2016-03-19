//! Module implementing evaluation of operator-related AST nodes.
//!
//! This includes unary and conditional operators, as well as binary operators
//! and their curried versions (which evaluate into functions).

use std::iter;

use eval::{self, api, Context, Eval, Value};
use eval::model::Invoke;
use eval::model::function::{Args, Function};
use eval::model::value::{ArrayRepr, FloatRepr, IntegerRepr, StringRepr};
use parse::ast::{Associativity, BinaryOpNode, ConditionalNode, CurriedBinaryOpNode, UnaryOpNode};


/// Evaluate the unary operator AST node.
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
        eval1!(arg : Boolean { !arg });
        UnaryOpNode::err("!", &arg)
    }

    /// Produce an error about invalid argument for an operator.
    fn err(op: &str, arg: &Value) -> eval::Result {
        Err(eval::Error::new(&format!(
            "invalid argument for `{}` operator: `{:?}`", op, arg
        )))
    }
}


/// Evaluate the binary operator AST node.
impl Eval for BinaryOpNode {
    fn eval(&self, context: &Context) -> eval::Result {
        match self.assoc {
            Associativity::Left => self.eval_left_assoc(&context),
            Associativity::Right => self.eval_right_assoc(&context),
        }
    }
}
impl BinaryOpNode {
    fn eval_left_assoc(&self, context: &Context) -> eval::Result {
        let mut result = try!(self.first.eval(&context));
        for &(ref op, ref arg) in &self.rest {
            let arg = try!(arg.eval(&context));
            result = try!(BinaryOpNode::eval_op(&op[..], result, arg, &context));
        }
        Ok(result)
    }

    fn eval_right_assoc(&self, context: &Context) -> eval::Result {
        unimplemented!()
    }

    fn eval_op(op: &str, left: Value, right: Value, context: &Context) -> eval::Result {
        match op {
            "<" => BinaryOpNode::eval_lt(left, right),
            "<=" => BinaryOpNode::eval_le(left, right),
            ">" => BinaryOpNode::eval_gt(left, right),
            ">=" => BinaryOpNode::eval_ge(left, right),
            "==" => BinaryOpNode::eval_eq(left, right),
            "!=" => BinaryOpNode::eval_ne(left, right),
            "@" => BinaryOpNode::eval_at(left, right),
            "&" => BinaryOpNode::eval_and(left, right),
            "$" => BinaryOpNode::eval_dollar(left, right, &context),
            "+" => BinaryOpNode::eval_plus(left, right),
            "-" => BinaryOpNode::eval_minus(left, right),
            "*" => BinaryOpNode::eval_times(left, right),
            "/" => BinaryOpNode::eval_by(left, right),
            "%" => BinaryOpNode::eval_modulo(left, right),
            "**" => BinaryOpNode::eval_power(left, right),
            _ => Err(eval::Error::new(&format!("unknown binary operator: `{}`", op))),
        }
    }
}

// Comparison operators.
impl BinaryOpNode {
    /// Evaluate the "<" operator for two values.
    fn eval_lt(left: Value, right: Value) -> eval::Result {
        eval2!((left: Integer, right: Integer) -> Boolean { left < right });
        eval2!((left: Integer, right: Float) -> Boolean { (left as FloatRepr) < right });
        eval2!((left: Float, right: Integer) -> Boolean { left < (right as FloatRepr) });
        eval2!((left: Float, right: Float) -> Boolean { left < right });
        BinaryOpNode::err("<", left, right)
    }

    /// Evaluate the "<=" operator for two values.
    fn eval_le(left: Value, right: Value) -> eval::Result {
        eval2!((left: Integer, right: Integer) -> Boolean { left <= right });
        eval2!((left: Integer, right: Float) -> Boolean { (left as FloatRepr) <= right });
        eval2!((left: Float, right: Integer) -> Boolean { left <= (right as FloatRepr) });
        eval2!((left: Float, right: Float) -> Boolean { left <= right });
        BinaryOpNode::err("<=", left, right)
    }

    /// Evaluate the ">" operator for two values.
    fn eval_gt(left: Value, right: Value) -> eval::Result {
        eval2!((left: Integer, right: Integer) -> Boolean { left > right });
        eval2!((left: Integer, right: Float) -> Boolean { (left as FloatRepr) > right });
        eval2!((left: Float, right: Integer) -> Boolean { left > (right as FloatRepr) });
        eval2!((left: Float, right: Float) -> Boolean { left > right });
        BinaryOpNode::err(">", left, right)
    }

    /// Evaluate the ">=" operator for two values.
    fn eval_ge(left: Value, right: Value) -> eval::Result {
        eval2!((left: Integer, right: Integer) -> Boolean { left >= right });
        eval2!((left: Integer, right: Float) -> Boolean { (left as FloatRepr) >= right });
        eval2!((left: Float, right: Integer) -> Boolean { left >= (right as FloatRepr) });
        eval2!((left: Float, right: Float) -> Boolean { left >= right });
        BinaryOpNode::err(">=", left, right)
    }

    /// Evaluate the "==" operator for two values.
    fn eval_eq(left: Value, right: Value) -> eval::Result {
        // numeric types
        eval2!((left: Integer, right: Integer) -> Boolean { left == right });
        eval2!((left: Integer, right: Float) -> Boolean { (left as FloatRepr) == right });
        eval2!((left: Float, right: Integer) -> Boolean { left == (right as FloatRepr) });
        eval2!((left: Float, right: Float) -> Boolean { left == right });

        // others
        eval2!((left: Boolean, right: Boolean) -> Boolean { left == right });
        eval2!((left: &String, right: &String) -> Boolean { left == right });
        eval2!((left: &Array, right: &Array) -> Boolean { left == right });
        eval2!((left: &Object, right: &Object) -> Boolean { left == right });

        BinaryOpNode::err("==", left, right)
    }

    /// Evaluate the "!=" operator for two values.
    fn eval_ne(left: Value, right: Value) -> eval::Result {
        // numeric types
        eval2!((left: Integer, right: Integer) -> Boolean { left != right });
        eval2!((left: Integer, right: Float) -> Boolean { (left as FloatRepr) != right });
        eval2!((left: Float, right: Integer) -> Boolean { left != (right as FloatRepr) });
        eval2!((left: Float, right: Float) -> Boolean { left != right });

        // others
        eval2!((left: Boolean, right: Boolean) -> Boolean { left != right });
        eval2!((left: &String, right: &String) -> Boolean { left != right });
        eval2!((left: &Array, right: &Array) -> Boolean { left != right });
        eval2!((left: &Object, right: &Object) -> Boolean { left != right });

        BinaryOpNode::err("!=", left, right)
    }

    /// Evaluate the "@" operator for two values.
    fn eval_at(left: Value, right: Value) -> eval::Result {
        // value @ array is a membership test
        if let &Value::Array(ref a) = &right {
            return Ok(Value::Boolean(a.contains(&left)));
        }

        BinaryOpNode::err("@", left, right)
    }
}

// Other binary operators.
impl BinaryOpNode {
    /// Evaluate the "&" operator for two values.
    fn eval_and(left: Value, right: Value) -> eval::Result {
        if left.is_function() && right.is_function() {
            let left = left.unwrap_function();
            let right = right.unwrap_function();
            return right.compose_with(left)  // reverse order!
                .map(Value::Function)
                .ok_or_else(|| eval::Error::new(&format!(
                    "second argument of `&` must be a unary function"
                )));
        }
        BinaryOpNode::err("&", left, right)
    }

    /// Evaluate the "$" operator for two values.
    fn eval_dollar(left: Value, right: Value, context: &Context) -> eval::Result {
        if left.is_function() {
            let left = left.unwrap_function();
            return if left.arity() == 1 {
                left.invoke(vec![right], &context)
            } else {
                left.curry(right)
                    .map(Value::Function)
                    .ok_or_else(|| eval::Error::new(&format!(
                        "left side of `$` must be a function taking at least one argument"
                    )))
            };
        }
        BinaryOpNode::err("$", left, right)
    }

    /// Evaluate the "+" operator for two values.
    fn eval_plus(left: Value, right: Value) -> eval::Result {
        eval2!(left, right : &String { left.clone() + &*right });
        eval2!(left, right : Integer { left + right });
        eval2!(left, right : Float { left + right });
        eval2!((left: Integer, right: Float) -> Float { left as FloatRepr + right });
        eval2!((left: Float, right: Integer) -> Float { left + right as FloatRepr });

        eval2!((left: &Array, right: &Array) -> Array {{
            let mut left = left.clone();
            let mut right = right.clone();
            left.append(&mut right);
            left
        }});
        eval2!((left: &Object, right: &Object) -> Object {{
            let mut left = left.clone();
            for (k, v) in right {
                left.insert(k.to_owned(), v.clone());
            }
            left
        }});

        BinaryOpNode::err("+", left, right)
    }

    /// Evaluate the "-" operator for two values.
    fn eval_minus(left: Value, right: Value) -> eval::Result {
        eval2!(left, right : Integer { left - right });
        eval2!(left, right : Float { left - right });
        eval2!((left: Integer, right: Float) -> Float { left as FloatRepr - right });
        eval2!((left: Float, right: Integer) -> Float { left - right as FloatRepr });
        BinaryOpNode::err("-", left, right)
    }

    /// Evaluate the "*" operator for two values.
    fn eval_times(left: Value, right: Value) -> eval::Result {
        eval2!(left, right : Integer { left * right });
        eval2!(left, right : Float { left * right });

        // multiplying string/array by a number is repeating (like in Python)
        eval2!((left: &String, right: Integer) -> String where (right > 0) {
            iter::repeat(left).map(StringRepr::clone).take(right as usize).collect()
        });
        eval2!((left: &Array, right: Integer) -> Array where (right > 0) {{
            iter::repeat(left).map(ArrayRepr::clone).take((right - 1) as usize)
                .fold(left.clone(), |mut res, mut next| { res.append(&mut next); res })
        }});

        // "multiplying" array by string means a join, with string as separator
        if left.is_array() && right.is_string() {
            return api::strings::join(left, right);
        }

        // "multiplying" functions is composition
        if left.is_function() && right.is_function() {
            let left = left.unwrap_function();
            let right = right.unwrap_function();
            return left.compose_with(right)
                .map(Value::Function)
                .ok_or_else(|| eval::Error::new(&format!(
                    "left side of function composition must be unary"
                )));
        }

        BinaryOpNode::err("*", left, right)
    }

    /// Evaluate the "/" operator for two values.
    fn eval_by(left: Value, right: Value) -> eval::Result {
        eval2!(left, right : Integer { left / right });
        eval2!(left, right : Float { left / right });

        // "dividing" string by string is a shorthand for split()
        if left.is_string() && right.is_string() {
            return api::strings::split(left, right);
        }

        BinaryOpNode::err("/", left, right)
    }

    /// Evaluate the "%" operator for two values.
    fn eval_modulo(left: Value, right: Value) -> eval::Result {
        // modulo/remainder
        eval2!(left, right : Integer { left % right });
        eval2!(left, right : Float { left % right });
        eval2!((left: Integer, right: Float) -> Float {
            (left as FloatRepr) % right
        });
        eval2!((left: Float, right: Integer) -> Float {
            left % (right as FloatRepr)
        });

        // string formatting (for just one argument (but it can be an array))
        if left.is_string() {
            return api::strings::format_(left, right);
        }

        BinaryOpNode::err("%", left, right)
    }

    /// Evaluate the "**" operator for two values.
    fn eval_power(left: Value, right: Value) -> eval::Result {
        eval2!(left, right : Integer {{
            if right > (u32::max_value() as IntegerRepr) {
                return Err(eval::Error::new(&format!(
                    "exponent out of range: {}", right
                )));
            }
            left.pow(right as u32)
        }});
        eval2!(left, right : Float { left.powf(right) });
        eval2!((left: Integer, right: Float) -> Float {
            (left as FloatRepr).powf(right)
        });
        eval2!((left: Float, right: Integer) -> Float {{
            if right > (i32::max_value() as IntegerRepr) {
                return Err(eval::Error::new(&format!(
                    "exponent out of range: {}", right
                )));
            }
            left.powi(right as i32)
        }});

        BinaryOpNode::err("**", left, right)
    }

    /// Produce an error about invalid arguments for an operator.
    fn err(op: &str, left: Value, right: Value) -> eval::Result {
        Err(eval::Error::new(&format!(
            "invalid arguments for `{}` operator: `{:?}` and `{:?}`",
            op, left, right)))
    }
}


/// Evaluate the curried binary operator node.
impl Eval for CurriedBinaryOpNode {
    fn eval(&self, context: &Context) -> eval::Result {
        if let Some(ref left) = self.left {
            let arg = try!(left.eval(&context));
            return self.eval_with_left(arg);
        }
        if let Some(ref right) = self.right {
            let arg = try!(right.eval(&context));
            return self.eval_with_right(arg);
        }
        self.eval_with_none(&context)
    }
}
impl CurriedBinaryOpNode {
    fn eval_with_left(&self, arg: Value) -> eval::Result {
        let op = self.op.clone();
        Ok(Value::Function(Function::from_native_ctx(1, move |args: Args, ctx: &Context| {
            let other = try!(CurriedBinaryOpNode::take_one_arg(args));
            BinaryOpNode::eval_op(&op, arg.clone(), other, &ctx)
        })))
    }
    fn eval_with_right(&self, arg: Value) -> eval::Result {
        let op = self.op.clone();
        Ok(Value::Function(Function::from_native_ctx(1, move |args: Args, ctx: &Context| {
            let other = try!(CurriedBinaryOpNode::take_one_arg(args));
            BinaryOpNode::eval_op(&op, other, arg.clone(), &ctx)
        })))
    }

    fn eval_with_none(&self, context: &Context) -> eval::Result {
        let op = self.op.clone();
        Ok(Value::Function(Function::from_native_ctx(2, move |args: Args, ctx: &Context| {
            if args.len() != 2 {
                return Err(eval::Error::new(&format!(
                    "invalid number of arguments: expected {}, got {}",
                    2, args.len()
                )));
            }
            let mut args = args.into_iter();
            BinaryOpNode::eval_op(&op, args.next().unwrap(), args.next().unwrap(), &ctx)
        })))
    }

    fn take_one_arg(args: Args) -> eval::Result {
        if args.len() != 1 {
            Err(eval::Error::new(&format!(
                "invalid number of arguments: expected {}, got {}",
                1, args.len()
            )))
        } else {
            Ok(args.into_iter().next().unwrap())
        }
    }
}


/// Evaluate the ternary operator / conditional node.
impl Eval for ConditionalNode {
    fn eval(&self, context: &Context) -> eval::Result {
        let cond = try!(self.cond.eval(&context));
        let cond_type = cond.typename();
        if let Value::Boolean(condition) = cond {
            if condition {
                self.then.eval(&context)
            } else {
                self.else_.eval(&context)
            }
        } else {
            Err(eval::Error::new(&format!(
                "expected a boolean condition, got {} instead", cond_type
            )))
        }
    }
}
