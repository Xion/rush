//! Module with built-in API that's available to the expressions.
//! This is basically the standard library of the language.

// NOTE: All actual API functions should be defined in submodules.

pub mod base;
pub mod conv;
pub mod math;
pub mod strings;


use std::f64;

use eval::{self, Context, Error, Value};
use eval::model::{Args, Arity, Function};
use eval::value::FloatRepr;


impl<'a> Context<'a> {
    /// Initialize symbols for the built-in functions and constants.
    /// This should be done only for the root Context (the one w/o a parent).
    pub fn init_builtins(&mut self) {
        assert!(self.is_root(), "Only root Context can have builtins!");
        self.init_functions();
        self.init_constants();
    }

    fn init_functions(&mut self) {
        //
        // Keep the list sorted alphabetically by function names.
        //
        self.define_unary(      "abs",      math::abs       );
        self.define_binary(     "after",    strings::after  );
        self.define_binary(     "before",   strings::before );
        self.define_unary(      "bool",     conv::bool      );
        self.define_unary(      "ceil",     math::ceil      );
        self.define_unary(      "exp",      math::exp       );
        self.define_binary_ctx( "filter",   base::filter    );
        self.define_unary(      "float",    conv::float     );
        self.define_unary(      "floor",    math::floor     );
        self.define_binary(     "format",   strings::format_);
        self.define_unary(      "int",      conv::int       );
        self.define_binary(     "join",     strings::join   );
        self.define_unary(      "len",      base::len       );
        self.define_unary(      "ln",       math::ln        );
        self.define_binary_ctx( "map",      base::map       );
        self.define_nullary(    "rand",     math::rand      );
        self.define_unary(      "re",       conv::regex     );
        self.define_unary(      "regex",    conv::regex     );
        self.define_unary(      "rev",      strings::rev    );
        self.define_unary(      "round",    math::round     );
        self.define_unary(      "sgn",      math::sgn       );
        self.define_binary(     "split",    strings::split  );
        self.define_unary(      "sqrt",     math::sqrt      );
        self.define_unary(      "str",      conv::str_      );
        self.define_ternary(    "sub",      strings::sub    );
        self.define_unary(      "trunc",    math::trunc     );
    }

    fn init_constants(&mut self) {
        //
        // Keep the list sorted alphabetically by constant names (ignore case).
        //
        self.set(   "Inf",      Value::Float(f64::INFINITY as FloatRepr));
        self.set(   "NaN",      Value::Float(f64::NAN as FloatRepr));
        self.set(   "pi",       Value::Float(f64::consts::PI as FloatRepr));
        //
        // Don't add "true" or "false", they are recognized at parser level!
        //
    }
}


// Helper methods for defining the "pure" API functions
// (those that don't access the Context directly).
#[allow(dead_code)]
impl<'a> Context<'a> {
    fn define<F>(&mut self, name: &'static str, arity: Arity, func: F) -> &mut Self
        where F: Fn(Args) -> eval::Result + 'static
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

    fn define_nullary<F>(&mut self, name: &'static str, func: F) -> &mut Self
        where F: Fn() -> eval::Result + 'static
    {
        self.define(name, Arity::Exact(0), move |_| { func() })
    }

    fn define_unary<F>(&mut self, name: &'static str, func: F) -> &mut Self
        where F: Fn(Value) -> eval::Result + 'static
    {
        self.define(name, Arity::Exact(1), move |args: Args| {
            let mut args = args.into_iter();
            func(args.next().unwrap())
        })
    }

    fn define_binary<F>(&mut self, name: &'static str, func: F) -> &mut Self
        where F: Fn(Value, Value) -> eval::Result + 'static
    {
        self.define(name, Arity::Exact(2), move |args: Args| {
            let mut args = args.into_iter();
            func(args.next().unwrap(), args.next().unwrap())
        })
    }

    fn define_ternary<F>(&mut self, name: &'static str, func: F) -> &mut Self
        where F: Fn(Value, Value, Value) -> eval::Result + 'static
    {
        self.define(name, Arity::Exact(3), move |args: Args| {
            let mut args = args.into_iter();
            func(args.next().unwrap(),
                 args.next().unwrap(),
                 args.next().unwrap())
        })
    }
}

// Helper methods for defining the API functions which access the Context.
#[allow(dead_code)]
impl<'a> Context<'a> {
    fn define_ctx<F>(&mut self, name: &'static str, arity: Arity, func: F) -> &mut Self
        where F: Fn(Args, &Context) -> eval::Result + 'static
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

    fn define_nullary_ctx<F>(&mut self, name: &'static str, func: F) -> &mut Self
        where F: Fn(&Context) -> eval::Result + 'static
    {
        self.define_ctx(name, Arity::Exact(0), move |_, context: &Context| {
            func(&context)
        })
    }

    fn define_unary_ctx<F>(&mut self, name: &'static str, func: F) -> &mut Self
        where F: Fn(Value, &Context) -> eval::Result + 'static
    {
        self.define_ctx(name, Arity::Exact(1), move |args: Args, context: &Context| {
            let mut args = args.into_iter();
            func(args.next().unwrap(), &context)
        })
    }

    fn define_binary_ctx<F>(&mut self, name: &'static str, func: F) -> &mut Self
        where F: Fn(Value, Value, &Context) -> eval::Result + 'static
    {
        self.define_ctx(name, Arity::Exact(2), move |args: Args, context: &Context| {
            let mut args = args.into_iter();
            func(args.next().unwrap(), args.next().unwrap(),
                &context)
        })
    }

    fn define_ternary_ctx<F>(&mut self, name: &'static str, func: F) -> &mut Self
        where F: Fn(Value, Value, Value, &Context) -> eval::Result + 'static
    {
        self.define_ctx(name, Arity::Exact(3), move |args: Args, context: &Context| {
            let mut args = args.into_iter();
            func(args.next().unwrap(),
                 args.next().unwrap(),
                 args.next().unwrap(),
                 &context)
        })
    }
}


/// Make sure a function got the correct number of arguments.
/// Usage:
///     try!(ensure_argcount("function", min, max));
///
fn ensure_argcount(name: &str, args: &Args, arity: Arity) -> Result<(), Error> {
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
