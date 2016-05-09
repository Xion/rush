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


// Methods for defining the "pure" API functions
// (those that don't access the Context directly).
impl<'c> Context<'c> {
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

    pub fn define_nullary<N:? Sized, F>(&mut self, name: &'static N, func: F) -> &mut Self
        where Name: Borrow<N>, N: ToOwned<Owned=Name> + Hash + Eq + Display,
              F: Fn() -> eval::Result + 'static
    {
        self.define(name, Arity::Exact(0), move |_| { func() })
    }
    pub fn define_nullary_plus<N: ?Sized, F>(&mut self, name: &'static N, func: F) -> &mut Self
        where Name: Borrow<N>, N: ToOwned<Owned=Name> + Hash + Eq + Display,
              F: Fn(Args) -> eval::Result + 'static
    {
        self.define(name, Arity::Minimum(0), func)
    }

    pub fn define_unary<N: ?Sized, F>(&mut self, name: &'static N, func: F) -> &mut Self
        where Name: Borrow<N>, N: ToOwned<Owned=Name> + Hash + Eq + Display,
              F: Fn(Value) -> eval::Result + 'static
    {
        self.define(name, Arity::Exact(1), move |args: Args| {
            let mut args = args.into_iter();
            func(args.next().unwrap())
        })
    }
    pub fn define_unary_plus<N: ?Sized, F>(&mut self, name: &'static N, func: F) -> &mut Self
        where Name: Borrow<N>, N: ToOwned<Owned=Name> + Hash + Eq + Display,
              F: Fn(Args) -> eval::Result + 'static
    {
        self.define(name, Arity::Minimum(1), func)
    }

    pub fn define_binary<N: ?Sized, F>(&mut self, name: &'static N, func: F) -> &mut Self
        where Name: Borrow<N>, N: ToOwned<Owned=Name> + Hash + Eq + Display,
              F: Fn(Value, Value) -> eval::Result + 'static
    {
        self.define(name, Arity::Exact(2), move |args: Args| {
            let mut args = args.into_iter();
            func(args.next().unwrap(), args.next().unwrap())
        })
    }
    pub fn define_binary_plus<N: ?Sized, F>(&mut self, name: &'static N, func: F) -> &mut Self
        where Name: Borrow<N>, N: ToOwned<Owned=Name> + Hash + Eq + Display,
              F: Fn(Args) -> eval::Result + 'static
    {
        self.define(name, Arity::Minimum(2), func)
    }

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
    pub fn define_ternary_plus<N: ?Sized, F>(&mut self, name: &'static N, func: F) -> &mut Self
        where Name: Borrow<N>, N: ToOwned<Owned=Name> + Hash + Eq + Display,
              F: Fn(Args) -> eval::Result + 'static
    {
        self.define(name, Arity::Minimum(3), func)
    }
}


// Methods for defining the API functions which access the Context.
impl<'c> Context<'c> {
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

    pub fn define_nullary_ctx<N: ?Sized, F>(&mut self, name: &'static N, func: F) -> &mut Self
        where Name: Borrow<N>, N: ToOwned<Owned=Name> + Hash + Eq + Display,
              F: Fn(&Context) -> eval::Result + 'static
    {
        self.define_ctx(name, Arity::Exact(0), move |_, context: &Context| {
            func(&context)
        })
    }
    pub fn define_nullary_plus_ctx<N: ?Sized, F>(&mut self, name: &'static N, func: F) -> &mut Self
        where Name: Borrow<N>, N: ToOwned<Owned=Name> + Hash + Eq + Display,
              F: Fn(Args, &Context) -> eval::Result + 'static
    {
        self.define_ctx(name, Arity::Minimum(0), func)
    }

    pub fn define_unary_ctx<N: ?Sized, F>(&mut self, name: &'static N, func: F) -> &mut Self
        where Name: Borrow<N>, N: ToOwned<Owned=Name> + Hash + Eq + Display,
              F: Fn(Value, &Context) -> eval::Result + 'static
    {
        self.define_ctx(name, Arity::Exact(1), move |args: Args, context: &Context| {
            let mut args = args.into_iter();
            func(args.next().unwrap(), &context)
        })
    }
    pub fn define_unary_plus_ctx<N: ?Sized, F>(&mut self, name: &'static N, func: F) -> &mut Self
        where Name: Borrow<N>, N: ToOwned<Owned=Name> + Hash + Eq + Display,
              F: Fn(Args, &Context) -> eval::Result + 'static
    {
        self.define_ctx(name, Arity::Minimum(1), func)
    }

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
    pub fn define_binary_plus_ctx<N: ?Sized, F>(&mut self, name: &'static N, func: F) -> &mut Self
        where Name: Borrow<N>, N: ToOwned<Owned=Name> + Hash + Eq + Display,
              F: Fn(Args, &Context) -> eval::Result + 'static
    {
        self.define_ctx(name, Arity::Minimum(2), func)
    }

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
    pub fn define_ternary_plus_ctx<N: ?Sized, F>(&mut self, name: &'static N, func: F) -> &mut Self
        where Name: Borrow<N>, N: ToOwned<Owned=Name> + Hash + Eq + Display,
              F: Fn(Args, &Context) -> eval::Result + 'static
    {
        self.define_ctx(name, Arity::Minimum(3), func)
    }
}


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
