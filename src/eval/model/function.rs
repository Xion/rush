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
    Native(Rc<NativeFunction>),

    /// Custom function that's been defined as part of the expression itself.
    Custom(CustomFunction),
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
            &Function::Custom(ref f) => {
                // TODO(xion): update accordingly when lambda syntax is finalized
                write!(fmt, "(:{}: <custom func>)", f.argnames.join(","))
            },
        }
    }
}

impl Invoke for Function {
    fn invoke(&self, args: Args, context: &Context) -> eval::Result {
        match self {
            &Function::Native(ref f) => f.invoke(args, context),
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


// Custom function type

/// Custom function type,
/// i.e. one that has been defined using the expression syntax.
#[derive(Clone)]
pub struct CustomFunction {
    pub argnames: Vec<String>,
    pub expr: Rc<Box<Eval>>,
}

impl CustomFunction {
    pub fn new(argnames: &[&str], expr: Box<Eval>) -> CustomFunction {
        CustomFunction{
            argnames: argnames.iter().map(|s| (*s).to_owned()).collect(),
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
            context.set_var(name, value);
        }
        self.expr.eval(&context)
    }
}
