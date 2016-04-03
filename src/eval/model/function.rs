//! Function type.
//!
//! This type represents both built-in functions (that are implemented directly
//! in native code), as well as user defined functions that are basically
//! pieces of AST.

use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::fmt;
use std::ops::{Add, Sub};
use std::rc::Rc;

use eval::{self, Context, Eval};
use super::value::Value;


/// Arguments to a function.
pub type Args = Vec<Value>;

/// Type for a number of arguments
/// (both expected by a function, and actually passed).
pub type ArgCount = usize;


/// Function arity (number of accepted arguments).
#[derive(Clone,Copy,Debug,PartialEq)]
pub enum Arity {
    /// Exact arity.
    /// Function requires the precise number of arguments, no more and no less.
    Exact(ArgCount),

    /// Minimum arity.
    /// Function requires at least that many arguments.
    Minimum(ArgCount),
}

impl Arity {
    #[inline(always)]
    pub fn is_exact(&self) -> bool {
        match *self { Arity::Exact(..) => true, _ => false }
    }

    /// Whether arity allows/accepts given argument count.
    /// This is equivalent to simple equality check: arity == argcount.
    #[inline]
    pub fn accepts(&self, argcount: ArgCount) -> bool {
        match *self {
            Arity::Exact(c) => argcount == c,
            Arity::Minimum(c) => argcount >= c,
        }
    }
}

impl fmt::Display for Arity {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Arity::Exact(c) => write!(fmt, "{}", c),
            Arity::Minimum(c) => write!(fmt, "{}+", c),
        }
    }
}

impl PartialOrd for Arity {
    /// Compare arities with each other.
    /// The ordering is only defined for exact arities.
    fn partial_cmp(&self, other: &Arity) -> Option<Ordering> {
        match *self {
            Arity::Exact(c1) => {
                if let Arity::Exact(c2) = *other {
                    return Some(c1.cmp(&c2));
                }
                None
            },
            _ => None,
        }
    }
}

impl PartialEq<ArgCount> for Arity {
    #[inline]
    fn eq(&self, count: &ArgCount) -> bool {
        if let Arity::Exact(c) = *self {
            return c == *count;
        }
        // Arity::Minimum always returns false to maintain transitivity
        // with the derived PartialEq<Arity>.
        false
    }
}
impl PartialOrd<ArgCount> for Arity {
    /// Compare arity with an actual argument count.
    ///
    /// Result indicates whether the count satisfies the arity, or whether
    /// more/fewer arguments would be needed.
    #[inline]
    fn partial_cmp(&self, count: &ArgCount) -> Option<Ordering> {
        match *self {
            Arity::Exact(c) => c.partial_cmp(&count),
            Arity::Minimum(c) => Some(
                // Once the argument count is above minimum,
                // it is "equal" for all intents and purposes.
                if *count >= c { Ordering::Equal } else { Ordering::Less }
            ),
        }
    }
}

impl Add<ArgCount> for Arity {
    type Output = Arity;

    /// Adding a specific argument count to an arity,
    /// equivalent to introducing that many new argument slots to a function.
    #[inline]
    fn add(self, rhs: ArgCount) -> Self::Output {
        match self {
            Arity::Exact(c) => Arity::Exact(c + rhs),
            Arity::Minimum(c) => Arity::Minimum(c), // no change
        }
    }
}
impl Sub<ArgCount> for Arity {
    type Output = Arity;

    /// Subtracting a specific argument count from an arity.
    /// Used to determine the new arity of a curried function.
    fn sub(self, rhs: ArgCount) -> Self::Output {
        match self {
            Arity::Exact(c) => {
                if c >= rhs {
                    return Arity::Exact(c - rhs);
                }
                panic!("underflow when subtracting from exact arity: {} - {} < 0",
                    c, rhs)
            },
            Arity::Minimum(c) => {
                if c > rhs {
                    return Arity::Minimum(c - rhs);
                } else if c == rhs {
                    return Arity::Exact(0);
                }
                panic!("underflow when subtracting from minimum arity: {} - {} < 0",
                    c, rhs)
            },
        }
    }
}


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
    /// An unspecified "invokable" object.
    Raw(Rc<Box<Invoke>>),

    /// Native function that's implemented in the interpreter.
    Native(Arity, Rc<NativeFunction>),

    /// Native function that's implemented in the interpreter
    /// and takes Context as an explicit parameter.
    NativeCtx(Arity, Rc<NativeCtxFunction>),

    /// Custom function that's been defined as part of the expression itself.
    Custom(CustomFunction),
}

impl Function {
    #[inline(always)]
    pub fn from_raw(invoke: Box<Invoke>) -> Function {
        Function::Raw(Rc::new(invoke))
    }
    #[inline(always)]
    pub fn from_native<F>(arity: Arity, f: F) -> Function
        where F: Fn(Args) -> eval::Result + 'static
    {
        Function::Native(arity, Rc::new(f))
    }
    #[inline(always)]
    pub fn from_native_ctx<F>(arity: Arity, f: F) -> Function
        where F: Fn(Args, &Context) -> eval::Result + 'static
    {
        Function::NativeCtx(arity, Rc::new(f))
    }
    #[inline(always)]
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
    #[inline(always)]
    fn eq(&self, _: &Self) -> bool {
        // for simplicity, functions are never equal to one another
        false
    }
}

impl fmt::Debug for Function {
    fn fmt(&self, fmt:  &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Function::Raw(ref f) => write!(fmt, "<raw func of {} arg(s)>", f.arity()),
            &Function::Native(a, _) => write!(fmt, "<native func of {} arg(s)>", a),
            &Function::NativeCtx(a, _) => write!(fmt, "<native(ctx) func of {} arg(s)>", a),
            &Function::Custom(ref f) => write!(fmt, "{:?}", f),
        }
    }
}

impl Invoke for Function {
    fn arity(&self) -> Arity {
        match self {
            &Function::Raw(ref f) => f.arity(),
            &Function::Native(a, _) => a,
            &Function::NativeCtx(a, _) => a,
            &Function::Custom(ref f) => f.arity(),
        }
    }

    fn invoke(&self, args: Args, context: &Context) -> eval::Result {
        match self {
            &Function::Raw(ref f) => f.invoke(args, &context),
            &Function::Native(_, ref f) => f(args),
            &Function::NativeCtx(_, ref f) => {
                let context = Context::with_parent(context);
                f(args, &context)
            },
            &Function::Custom(ref f) => f.invoke(args, &context),
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
    #[inline(always)]
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
        Arity::Exact(self.argnames.len())
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
