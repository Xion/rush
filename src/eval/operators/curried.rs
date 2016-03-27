//! Module implementing evaluation of curried binary operator AST nodes.

use eval::{self, Eval, Context, Value};
use eval::model::function::{Args, ArgCount, Arity, Function};
use parse::ast::{BinaryOpNode, CurriedBinaryOpNode};


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
            let other = try!(take_one_arg(args));
            BinaryOpNode::eval_op(&op, arg.clone(), other, &ctx)
        };
        Ok(Value::Function(Function::from_native_ctx(Arity::Exact(1), func)))
    }

    fn eval_with_right(&self, arg: Value) -> eval::Result {
        let op = self.op.clone();
        let func = move |args: Args, ctx: &Context| {
            let other = try!(take_one_arg(args));
            BinaryOpNode::eval_op(&op, other, arg.clone(), &ctx)
        };
        Ok(Value::Function(Function::from_native_ctx(Arity::Exact(1), func)))
    }

    fn eval_with_none(&self) -> eval::Result {
        let op = self.op.clone();
        let func = move |args: Args, ctx: &Context| {
            let (left, right) = try!(take_two_args(args));
            BinaryOpNode::eval_op(&op, left, right, &ctx)
        };
        Ok(Value::Function(Function::from_native_ctx(Arity::Exact(2), func)))
    }
}


// Utility functions

fn take_one_arg(args: Args) -> eval::Result {
    try!(ensure_argcount(&args, 1));
    Ok(args.into_iter().next().unwrap())
}
fn take_two_args(args: Args) -> Result<(Value, Value), eval::Error> {
    try!(ensure_argcount(&args, 2));
    let mut args = args.into_iter();
    Ok((args.next().unwrap(), args.next().unwrap()))
}

fn ensure_argcount(args: &Args, count: ArgCount) -> Result<(), eval::Error> {
    if args.len() != count {
        return Err(eval::Error::new(&format!(
            "invalid number of arguments: expected {}, got {}",
            count, args.len()
        )));
    }
    Ok(())
}
