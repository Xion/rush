//! Function type.
//!
//! This type represents both built-in functions (that are implemented directly
//! in native code), as well as user defined functions that are basically
//! pieces of AST.

use std::cmp::PartialEq;
use std::fmt;
use std::rc::Rc;

use eval::{self, Context, Eval};
use super::value::Value;


/// Denotes an object that works as a callable function within an expression.
///
/// (This isn't named Call because call() function would conflict with
/// the quasi-intrinsic method on Fn types in Rust).
pub trait Invoke {
    fn invoke(&self, args: Args, context: &Context) -> eval::Result;
}


/// Function that can be invoked when evaluating an expression.
#[derive(Clone)]
pub enum Function {
    /// Native function that's implemented in the interpreter.
    Native(Rc<Box<NativeFunction>>),

    /// Native function that's implemented in the interpreter
    /// and takes Context as an explicit parameter.
    NativeCtx(Rc<Box<NativeCtxFunction>>),

    /// Custom function that's been defined as part of the expression itself.
    Custom(CustomFunction),
}

#[allow(dead_code)]
impl Function {
    pub fn from_native<F>(f: F) -> Function
        where F: Fn(Args) -> eval::Result + 'static
    {
        Function::Native(Rc::new(Box::new(f)))
    }
    pub fn from_boxed_native(f: Box<NativeFunction>) -> Function {
        Function::Native(Rc::new(f))
    }

    pub fn from_native_ctx<F>(f: F) -> Function
        where F: Fn(Args, &Context) -> eval::Result + 'static
    {
        Function::NativeCtx(Rc::new(Box::new(f)))
    }
    pub fn from_boxed_native_ctx(f: Box<NativeCtxFunction>) -> Function {
        Function::NativeCtx(Rc::new(f))
    }

    pub fn from_lambda(argnames: Vec<String>, expr: Box<Eval>) -> Function {
        Function::Custom(CustomFunction::new(argnames, expr))
    }

    /// Function composition:
    /// self.compose_with(other)(x) === self(other(x))
    pub fn compose_with(self, other: Function) -> Function {
        // TODO(xion): when function arity is stored and known,
        // check that `self` is unary and return an error if not so
        let result =  Box::new(move |args, context: &Context| {
            let intermediate = try!(other.invoke(args, &context));
            self.invoke(vec![intermediate], &context)
        });
        Function::from_boxed_native_ctx(result)
    }
}

impl PartialEq for Function {
    fn eq(&self, _: &Self) -> bool {
        // for simplicity, functions are never equal to one another
        false
    }
}

impl fmt::Debug for Function {
    fn fmt(&self, fmt:  &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Function::Native(..) => write!(fmt, "<native func>"),
            &Function::NativeCtx(..) => write!(fmt, "<native(ctx) func>"),
            &Function::Custom(ref f) => {
                write!(fmt, "(|{}| <custom func>)", f.argnames.join(","))
            },
        }
    }
}

impl Invoke for Function {
    fn invoke(&self, args: Args, context: &Context) -> eval::Result {
        match self {
            &Function::Native(ref f) => f.invoke(args, context),
            &Function::NativeCtx(ref f) => f.invoke(args, context),
            &Function::Custom(ref f) => f.invoke(args, context),
        }
    }
}


// Native function type

/// Arguments to a function.
pub type Args = Vec<Value>;

/// Native function type,
/// i.e. one that's implemented in the interpreter itself.
pub type NativeFunction = Fn(Args) -> eval::Result;

impl Invoke for NativeFunction {
    fn invoke(&self, args: Args, _: &Context) -> eval::Result {
        self(args)
    }
}

/// Native function that directly operates on its Context.
pub type NativeCtxFunction = Fn(Args, &Context) -> eval::Result;

impl Invoke for NativeCtxFunction {
    fn invoke(&self, args: Args, context: &Context) -> eval::Result {
        self(args, &context)
    }
}


// Custom function type

/// Custom function type,
/// i.e. one that has been defined using the expression syntax.
#[derive(Clone)]
pub struct CustomFunction {
    pub argnames: Vec<String>,
    pub expr: Rc<Box<Eval>>,
}

impl CustomFunction {
    pub fn new(argnames: Vec<String>, expr: Box<Eval>) -> CustomFunction {
        CustomFunction{
            argnames: argnames,
            expr: Rc::new(expr),
        }
    }
}

impl Invoke for CustomFunction {
    fn invoke(&self, mut args: Args, context: &Context) -> eval::Result {
        let expected_count = self.argnames.len();
        let actual_count = args.len();
        if actual_count != expected_count {
            return Err(eval::Error::new(&format!(
                "function expects {} argument(s), got {}",
                expected_count, actual_count
            )));
        }

        let mut context = Context::with_parent(context);
        for (name, value) in self.argnames.iter().zip(args.drain(..)) {
            context.set(name, value);
        }
        self.expr.eval(&context)
    }
}
