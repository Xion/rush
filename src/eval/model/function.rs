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


/// Arguments to a function.
pub type Args = Vec<Value>;

/// Function arity (number of accepted arguments).
pub type Arity = usize;
// TODO(xion): make this an enum with Exact and Minimum, to allow for varargs;
// currying of variadic functions with $ operator shall result in invocation
// as soon as number of accumulated arguments reaches Minimum


/// Denotes an object that works as a callable function within an expression.
///
/// (This isn't named Call because call() function would conflict with
/// the quasi-intrinsic method on Fn types in Rust).
pub trait Invoke {
    fn arity(&self) -> Arity;
    fn invoke(&self, args: Args, context: &Context) -> eval::Result;
}


/// Function that can be invoked when evaluating an expression.
#[derive(Clone)]
pub enum Function {
    /// Native function that's implemented in the interpreter.
    Native(Arity, Rc<NativeFunction>),

    /// Native function that's implemented in the interpreter
    /// and takes Context as an explicit parameter.
    NativeCtx(Arity, Rc<NativeCtxFunction>),

    /// Custom function that's been defined as part of the expression itself.
    Custom(CustomFunction),
}

impl Function {
    pub fn from_native<F>(arity: Arity, f: F) -> Function
        where F: Fn(Args) -> eval::Result + 'static
    {
        Function::Native(arity, Rc::new(f))
    }
    pub fn from_native_ctx<F>(arity: Arity, f: F) -> Function
        where F: Fn(Args, &Context) -> eval::Result + 'static
    {
        Function::NativeCtx(arity, Rc::new(f))
    }
    pub fn from_lambda(argnames: Vec<String>, expr: Box<Eval>) -> Function {
        Function::Custom(CustomFunction::new(argnames, expr))
    }

    /// Function composition:
    /// self.compose_with(other)(x) === self(other(x))
    #[inline]
    pub fn compose_with(self, other: Function) -> Option<Function> {
        if self.arity() == 1 {
            let arity = other.arity();
            let result = move |args, context: &Context| {
                let intermediate = try!(other.invoke(args, &context));
                self.invoke(vec![intermediate], &context)
            };
            return Some(Function::from_native_ctx(arity, result));
        }
        None
    }

    /// Function currying (partial application):
    /// self.curry(arg)(x) === self(arg, x)
    #[inline]
    pub fn curry(self, arg: Value) -> Option<Function> {
        if self.arity() >= 1 {
            let arity = self.arity() - 1;
            let result = move |mut args: Args, context: &Context| {
                args.insert(0, arg.clone());
                self.invoke(args, &context)
            };
            return Some(Function::from_native_ctx(arity, result));
        }
        None
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
            &Function::Native(a, _) => write!(fmt, "<native func of {} arg(s)>", a),
            &Function::NativeCtx(a, _) => write!(fmt, "<native(ctx) func of {} arg(s)>", a),
            &Function::Custom(ref f) => write!(fmt, "{:?}", f),
        }
    }
}

impl Invoke for Function {
    fn arity(&self) -> Arity {
        match self {
            &Function::Native(a, _) => a,
            &Function::NativeCtx(a, _) => a,
            &Function::Custom(ref f) => f.arity(),
        }
    }

    fn invoke(&self, args: Args, context: &Context) -> eval::Result {
        match self {
            &Function::Native(_, ref f) => f(args),
            &Function::NativeCtx(_, ref f) => f(args, &context),
            &Function::Custom(ref f) => f.invoke(args, context),
        }
    }
}


// Function types

/// Native function type,
/// i.e. one that's implemented in the interpreter itself.
pub type NativeFunction = Fn(Args) -> eval::Result;


/// Native function that directly operates on its Context.
pub type NativeCtxFunction = Fn(Args, &Context) -> eval::Result;


/// Custom function type,
/// i.e. one that has been defined using the expression syntax.
#[derive(Clone)]
pub struct CustomFunction {
    argnames: Vec<String>,
    expr: Rc<Box<Eval>>,
}

impl CustomFunction {
    pub fn new(argnames: Vec<String>, expr: Box<Eval>) -> CustomFunction {
        CustomFunction{
            argnames: argnames,
            expr: Rc::new(expr),
        }
    }
}

impl fmt::Debug for CustomFunction {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "|{}| {:?}", self.argnames.join(","), self.expr)
    }
}

impl Invoke for CustomFunction {
    #[inline(always)]
    fn arity(&self) -> Arity {
        self.argnames.len()
    }

    fn invoke(&self, args: Args, context: &Context) -> eval::Result {
        let expected_count = self.argnames.len();
        let actual_count = args.len();
        if actual_count != expected_count {
            return Err(eval::Error::new(&format!(
                "function expects {} argument(s), got {}",
                expected_count, actual_count
            )));
        }

        let mut context = Context::with_parent(context);
        for (name, value) in self.argnames.iter().zip(args.into_iter()) {
            context.set(name, value);
        }
        self.expr.eval(&context)
    }
}
