//! Module implementing evaluation of operator-related AST nodes.
//!
//! This includes unary and conditional operators, as well as binary operators
//! and their curried versions (which evaluate into functions).

mod unary;
mod binary;


use eval::{self, api, Context, Eval, Value};
use eval::model::function::{Args, Arity, Function};
use parse::ast::{BinaryOpNode, ConditionalNode, CurriedBinaryOpNode};


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
        self.eval_with_none()
    }
}
impl CurriedBinaryOpNode {
    fn eval_with_left(&self, arg: Value) -> eval::Result {
        let op = self.op.clone();
        let func = move |args: Args, ctx: &Context| {
            let other = try!(CurriedBinaryOpNode::take_one_arg(args));
            BinaryOpNode::eval_op(&op, arg.clone(), other, &ctx)
        };
        Ok(Value::Function(Function::from_native_ctx(Arity::Exact(1), func)))
    }
    fn eval_with_right(&self, arg: Value) -> eval::Result {
        let op = self.op.clone();
        let func = move |args: Args, ctx: &Context| {
            let other = try!(CurriedBinaryOpNode::take_one_arg(args));
            BinaryOpNode::eval_op(&op, other, arg.clone(), &ctx)
        };
        Ok(Value::Function(Function::from_native_ctx(Arity::Exact(1), func)))
    }

    fn eval_with_none(&self) -> eval::Result {
        let op = self.op.clone();
        let func = move |args: Args, ctx: &Context| {
            if args.len() != 2 {
                return Err(eval::Error::new(&format!(
                    "invalid number of arguments: expected {}, got {}",
                    2, args.len()
                )));
            }
            let mut args = args.into_iter();
            BinaryOpNode::eval_op(&op, args.next().unwrap(), args.next().unwrap(), &ctx)
        };
        Ok(Value::Function(Function::from_native_ctx(Arity::Exact(2), func)))
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
    #[inline]
    fn eval(&self, context: &Context) -> eval::Result {
        let condition = try!(
            self.cond.eval(&context).and_then(api::conv::bool)
        ).unwrap_bool();
        if condition {
            self.then.eval(&context)
        } else {
            self.else_.eval(&context)
        }
    }
}
