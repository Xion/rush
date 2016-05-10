//! Context methods for defining API functions.
//!
//! These methods are used by eval::api module to expose the built-in functions
//! and make them available to expressions.

#![allow(dead_code)]

use std::borrow::{Borrow, ToOwned};
use std::fmt::Display;
use std::hash::Hash;

use eval::model::{Args, Arity, Function, Name};
use eval::{self, Context, Error, Value};


// Base methods for defining functions with any arity.
impl<'c> Context<'c> {
    /// Define a regular function with given arity, implemented using native Rust function.
    ///
    /// Functions defined this way may only use their arguments (and arbirary Rust code,
    /// of course) to compute their result. One of their limitations is that they cannot
    /// indirectly invoke other functions (i.e. those that are only represented as Value objects).
    ///
    /// Returns a reference to the Context for easy chaining.
    ///
    pub fn define<'n, N: ?Sized, F>(&mut self, name: &'static N, arity: Arity, func: F) -> &mut Self
        where Name: Borrow<N>, N: ToOwned<Owned=Name> + Hash + Eq + Display,
              F: Fn(Args) -> eval::Result + 'static
    {
        assert!(!self.is_defined_here(name),
             "`{}` has already been defined in this Context!", name);

        let function = Function::from_native(arity, move |args: Args| {
            try!(ensure_argcount(name, &args, arity));
            func(args)
        });
        self.set(name, Value::Function(function));
        self
    }

    /// Define a contextualized function with given arity, implemented using a Rust functgion.
    ///
    /// Contextualized functions receive, as their last argument, an (immutable) reference
    /// to the Context they are invoked in. (This may be a child Context of the one they're
    /// defined in). This allows them to indirectly invoke other functions via the Invoke trait.
    ///
    /// Returns a reference to the Context for easy chaining.
    ///
    pub fn define_ctx<N: ?Sized, F>(&mut self, name: &'static N, arity: Arity, func: F) -> &mut Self
        where Name: Borrow<N>, N: ToOwned<Owned=Name> + Hash + Eq + Display,
              F: Fn(Args, &Context) -> eval::Result + 'static
    {
        assert!(!self.is_defined_here(name),
             "`{}` has already been defined in this Context!", name);

        let function = Function::from_native_ctx(arity, move |args: Args, context: &Context| {
            try!(ensure_argcount(name, &args, arity));
            func(args, &context)
        });
        self.set(name, Value::Function(function));
        self
    }
}


//
// Methods for defining functions taking at least 0 arguments.
//
impl<'c> Context<'c> {
    /// Define a regular function taking no arguments.
    pub fn define_nullary<N:? Sized, F>(&mut self, name: &'static N, func: F) -> &mut Self
        where Name: Borrow<N>, N: ToOwned<Owned=Name> + Hash + Eq + Display,
              F: Fn() -> eval::Result + 'static
    {
        self.define(name, Arity::Exact(0), move |_| { func() })
    }

    /// Define a regular function taking zero or more arguments.
    pub fn define_nullary_plus<N: ?Sized, F>(&mut self, name: &'static N, func: F) -> &mut Self
        where Name: Borrow<N>, N: ToOwned<Owned=Name> + Hash + Eq + Display,
              F: Fn(Args) -> eval::Result + 'static
    {
        self.define(name, Arity::Minimum(0), func)
    }

    /// Define a contextualized function taking no arguments.
    pub fn define_nullary_ctx<N: ?Sized, F>(&mut self, name: &'static N, func: F) -> &mut Self
        where Name: Borrow<N>, N: ToOwned<Owned=Name> + Hash + Eq + Display,
              F: Fn(&Context) -> eval::Result + 'static
    {
        self.define_ctx(name, Arity::Exact(0), move |_, context: &Context| {
            func(&context)
        })
    }

    /// Define a contextualized function taking zero or more arguments.
    pub fn define_nullary_plus_ctx<N: ?Sized, F>(&mut self, name: &'static N, func: F) -> &mut Self
        where Name: Borrow<N>, N: ToOwned<Owned=Name> + Hash + Eq + Display,
              F: Fn(Args, &Context) -> eval::Result + 'static
    {
        self.define_ctx(name, Arity::Minimum(0), func)
    }
}


//
// Methods for defining functions taking mostly 1 argument.
//
impl<'c> Context<'c> {
    /// Define a regular function taking exactly one argument.
    pub fn define_unary<N: ?Sized, F>(&mut self, name: &'static N, func: F) -> &mut Self
        where Name: Borrow<N>, N: ToOwned<Owned=Name> + Hash + Eq + Display,
              F: Fn(Value) -> eval::Result + 'static
    {
        self.define(name, Arity::Exact(1), move |args: Args| {
            let mut args = args.into_iter();
            func(args.next().unwrap())
        })
    }

    /// Define a regular function taking one or more arguments.
    pub fn define_unary_plus<N: ?Sized, F>(&mut self, name: &'static N, func: F) -> &mut Self
        where Name: Borrow<N>, N: ToOwned<Owned=Name> + Hash + Eq + Display,
              F: Fn(Args) -> eval::Result + 'static
    {
        self.define(name, Arity::Minimum(1), func)
    }

    /// Define a contextualized function taking exactly one argument.
    pub fn define_unary_ctx<N: ?Sized, F>(&mut self, name: &'static N, func: F) -> &mut Self
        where Name: Borrow<N>, N: ToOwned<Owned=Name> + Hash + Eq + Display,
              F: Fn(Value, &Context) -> eval::Result + 'static
    {
        self.define_ctx(name, Arity::Exact(1), move |args: Args, context: &Context| {
            let mut args = args.into_iter();
            func(args.next().unwrap(), &context)
        })
    }

    /// Define a contextualized function taking one or more arguments.
    pub fn define_unary_plus_ctx<N: ?Sized, F>(&mut self, name: &'static N, func: F) -> &mut Self
        where Name: Borrow<N>, N: ToOwned<Owned=Name> + Hash + Eq + Display,
              F: Fn(Args, &Context) -> eval::Result + 'static
    {
        self.define_ctx(name, Arity::Minimum(1), func)
    }
}

impl<'c> Context<'c> {
    /// Define a regular function taking at most one argument.
    pub fn define_upto_unary<N: ?Sized, F>(&mut self, name: &'static N, func: F) -> &mut Self
        where Name: Borrow<N>, N: ToOwned<Owned=Name> + Hash + Eq + Display,
              F: Fn(Option<Value>) -> eval::Result + 'static
    {
        self.define(name, Arity::Range(0, 1), move |args: Args| {
            let mut args = args.into_iter();
            func(args.next())
        })
    }

    /// Define a contextualized function taking at most one argument.
    pub fn define_upto_unary_ctx<N: ?Sized, F>(&mut self, name: &'static N, func: F) -> &mut Self
        where Name: Borrow<N>, N: ToOwned<Owned=Name> + Hash + Eq + Display,
              F: Fn(Option<Value>, &Context) -> eval::Result + 'static
    {
        self.define_ctx(name, Arity::Range(0, 1), move |args: Args, context: &Context| {
            let mut args = args.into_iter();
            func(args.next(), &context)
        })
    }
}


//
// Methods for defining functions taking mostly 2 arguments.
//
impl<'c> Context<'c> {
    /// Define a regular function taking exactly two arguments.
    pub fn define_binary<N: ?Sized, F>(&mut self, name: &'static N, func: F) -> &mut Self
        where Name: Borrow<N>, N: ToOwned<Owned=Name> + Hash + Eq + Display,
              F: Fn(Value, Value) -> eval::Result + 'static
    {
        self.define(name, Arity::Exact(2), move |args: Args| {
            let mut args = args.into_iter();
            func(args.next().unwrap(), args.next().unwrap())
        })
    }

    /// Define a regular function taking two or more arguments.
    pub fn define_binary_plus<N: ?Sized, F>(&mut self, name: &'static N, func: F) -> &mut Self
        where Name: Borrow<N>, N: ToOwned<Owned=Name> + Hash + Eq + Display,
              F: Fn(Args) -> eval::Result + 'static
    {
        self.define(name, Arity::Minimum(2), func)
    }

    /// Define a contextualized function taking exactly two arguments.
    pub fn define_binary_ctx<N: ?Sized, F>(&mut self, name: &'static N, func: F) -> &mut Self
        where Name: Borrow<N>, N: ToOwned<Owned=Name> + Hash + Eq + Display,
              F: Fn(Value, Value, &Context) -> eval::Result + 'static
    {
        self.define_ctx(name, Arity::Exact(2), move |args: Args, context: &Context| {
            let mut args = args.into_iter();
            func(args.next().unwrap(), args.next().unwrap(),
                &context)
        })
    }

    /// Define a contextualized function taking two or more arguments.
    pub fn define_binary_plus_ctx<N: ?Sized, F>(&mut self, name: &'static N, func: F) -> &mut Self
        where Name: Borrow<N>, N: ToOwned<Owned=Name> + Hash + Eq + Display,
              F: Fn(Args, &Context) -> eval::Result + 'static
    {
        self.define_ctx(name, Arity::Minimum(2), func)
    }
}

impl<'c> Context<'c> {
    /// Define a regular function taking at most two arguments.
    pub fn define_upto_binary<N: ?Sized, F>(&mut self, name: &'static N, func: F) -> &mut Self
        where Name: Borrow<N>, N: ToOwned<Owned=Name> + Hash + Eq + Display,
              F: Fn(Option<Value>, Option<Value>) -> eval::Result + 'static
    {
        self.define(name, Arity::Range(0, 2), move |args: Args| {
            let mut opt_args = expand_vec(args, 2).into_iter();
            func(opt_args.next().unwrap(), opt_args.next().unwrap())
        })
    }

    /// Define a contextualized function taking at most two arguments.
    pub fn define_upto_binary_ctx<N: ?Sized, F>(&mut self, name: &'static N, func: F) -> &mut Self
        where Name: Borrow<N>, N: ToOwned<Owned=Name> + Hash + Eq + Display,
              F: Fn(Option<Value>, Option<Value>, &Context) -> eval::Result + 'static
    {
        self.define_ctx(name, Arity::Range(0, 2), move |args: Args, context: &Context| {
            let mut opt_args = expand_vec(args, 2).into_iter();
            func(opt_args.next().unwrap(), opt_args.next().unwrap(),
                 &context)
        })
    }
}


//
// Methods for defining functions taking mostly 3 arguments.
//
impl<'c> Context<'c> {
    /// Define a regular function taking exactly three arguments.
    pub fn define_ternary<N: ?Sized, F>(&mut self, name: &'static N, func: F) -> &mut Self
        where Name: Borrow<N>, N: ToOwned<Owned=Name> + Hash + Eq + Display,
              F: Fn(Value, Value, Value) -> eval::Result + 'static
    {
        self.define(name, Arity::Exact(3), move |args: Args| {
            let mut args = args.into_iter();
            func(args.next().unwrap(),
                 args.next().unwrap(),
                 args.next().unwrap())
        })
    }

    /// Define a regular function taking three or more arguments.
    pub fn define_ternary_plus<N: ?Sized, F>(&mut self, name: &'static N, func: F) -> &mut Self
        where Name: Borrow<N>, N: ToOwned<Owned=Name> + Hash + Eq + Display,
              F: Fn(Args) -> eval::Result + 'static
    {
        self.define(name, Arity::Minimum(3), func)
    }

    /// Define a contextualized function taking exactly three arguments.
    pub fn define_ternary_ctx<N: ?Sized, F>(&mut self, name: &'static N, func: F) -> &mut Self
        where Name: Borrow<N>, N: ToOwned<Owned=Name> + Hash + Eq + Display,
              F: Fn(Value, Value, Value, &Context) -> eval::Result + 'static
    {
        self.define_ctx(name, Arity::Exact(3), move |args: Args, context: &Context| {
            let mut args = args.into_iter();
            func(args.next().unwrap(),
                 args.next().unwrap(),
                 args.next().unwrap(),
                 &context)
        })
    }

    /// Define a contextualized function taking three or more arguments.
    pub fn define_ternary_plus_ctx<N: ?Sized, F>(&mut self, name: &'static N, func: F) -> &mut Self
        where Name: Borrow<N>, N: ToOwned<Owned=Name> + Hash + Eq + Display,
              F: Fn(Args, &Context) -> eval::Result + 'static
    {
        self.define_ctx(name, Arity::Minimum(3), func)
    }
}

impl<'c> Context<'c> {
    /// Define a regular function taking at most three arguments.
    pub fn define_upto_ternary<N: ?Sized, F>(&mut self, name: &'static N, func: F) -> &mut Self
        where Name: Borrow<N>, N: ToOwned<Owned=Name> + Hash + Eq + Display,
              F: Fn(Option<Value>, Option<Value>, Option<Value>) -> eval::Result + 'static
    {
        self.define(name, Arity::Range(0, 3), move |args: Args| {
            let mut opt_args = expand_vec(args, 3).into_iter();
            func(opt_args.next().unwrap(),
                 opt_args.next().unwrap(),
                 opt_args.next().unwrap())
        })
    }

    /// Define a contextualized function taking at most three arguments.
    pub fn define_upto_ternary_ctx<N: ?Sized, F>(&mut self, name: &'static N, func: F) -> &mut Self
        where Name: Borrow<N>, N: ToOwned<Owned=Name> + Hash + Eq + Display,
              F: Fn(Option<Value>, Option<Value>, Option<Value>, &Context) -> eval::Result + 'static
    {
        self.define_ctx(name, Arity::Range(0, 3), move |args: Args, context: &Context| {
            let mut opt_args = expand_vec(args, 3).into_iter();
            func(opt_args.next().unwrap(),
                 opt_args.next().unwrap(),
                 opt_args.next().unwrap(),
                 &context)
        })
    }
}


// Utility functions

/// Make sure a function got the correct number of arguments.
/// Usage:
///     try!(ensure_argcount("function", min, max));
///
fn ensure_argcount<N: ?Sized>(name: &N, args: &Args, arity: Arity) -> Result<(), Error>
    where N: Display
{
    let count = args.len();
    if arity.accepts(count) {
        Ok(())
    } else {
        Err(Error::new(&format!(
            "invalid number of arguments to {}(): expected {}, got {}",
            name, arity, count
        )))
    }
}

/// Expand a vector to given size by turning it into a vector of Options.
/// Additional items created by this expansion are set to None.
fn expand_vec<T>(v: Vec<T>, size: usize) -> Vec<Option<T>> {
    let len = v.len();
    assert!(size >= len,
        "Cannot expand a vector of len={} to a smaller size of {}", len, size);

    // don't use Vec::resize because it requires T: Clone and we don't want to clone needlessly
    let mut result: Vec<Option<T>> = Vec::with_capacity(size);
    for x in v {
        result.push(Some(x))
    }
    for _ in 0..size - len {
        result.push(None)
    }
    result
}
