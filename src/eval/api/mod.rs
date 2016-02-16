//! Module with built-in API that's available to the expressions.
//! This is basically the standard library of the language.

// NOTE: All actual API functions should be defined in submodules.

mod base;
mod conv;
mod math;
mod strings;


use eval::{self, Context, Error, Value};
use eval::model::{Args, Function};


impl<'a> Context<'a> {
    /// Initialize symbols corresponding to the built-in functions.
    /// This should be done only for the root Context (the one w/o a parent).
    pub fn init_builtins(&mut self) {
        assert!(self.is_root(), "Only root Context can have builtins!");

        //
        // Keep the list sorted alphabetically by function names.
        //
        self.define_unary(      "abs",      math::abs       );
        self.define_unary(      "bool",     conv::bool      );
        self.define_unary(      "float",    conv::float     );
        self.define_unary(      "int",      conv::int       );
        self.define_binary(     "join",     strings::join   );
        self.define_unary(      "len",      base::len       );
        self.define_nullary(    "rand",     math::rand      );
        self.define_unary(      "rev",      strings::rev    );
        self.define_unary(      "sgn",      math::sgn       );
        self.define_binary(     "split",    strings::split  );
        self.define_unary(      "sqrt",     math::sqrt      );
        self.define_unary(      "str",      conv::str_      );
        self.define_ternary(    "sub",      strings::sub    );
    }
}


// Helper methods for defining the API functions within Context.
impl<'a> Context<'a> {
    fn define<F>(&mut self, name: &str, func: F) -> &mut Self
        where F: Fn(Args) -> eval::Result + 'static
    {
        self.set(name, Value::Function(Function::from_native(func)));
        self
    }

    fn define_nullary<F>(&mut self, name: &'static str, func: F) -> &mut Self
        where F: Fn() -> eval::Result + 'static
    {
        self.define(name, move |args: Args| {
            try!(ensure_argcount(name, &args, 0, 0));
            func()
        })
    }

    fn define_unary<F>(&mut self, name: &'static str, func: F) -> &mut Self
        where F: Fn(Value) -> eval::Result + 'static
    {
        self.define(name, move |mut args: Args| {
            try!(ensure_argcount(name, &args, 1, 1));
            let mut args = args.drain(..);
            func(args.next().unwrap())
        })
    }

    fn define_binary<F>(&mut self, name: &'static str, func: F) -> &mut Self
        where F: Fn(Value, Value) -> eval::Result + 'static
    {
        self.define(name, move |mut args: Args| {
            try!(ensure_argcount(name, &args, 2, 2));
            let mut args = args.drain(..);
            func(args.next().unwrap(), args.next().unwrap())
        })
    }

    fn define_ternary<F>(&mut self, name: &'static str, func: F) -> &mut Self
        where F: Fn(Value, Value, Value) -> eval::Result + 'static
    {
        self.define(name, move |mut args: Args| {
            try!(ensure_argcount(name, &args, 3, 3));
            let mut args = args.drain(..);
            func(args.next().unwrap(),
                 args.next().unwrap(),
                 args.next().unwrap())
        })
    }
}


/// Make sure a function got the correct number of arguments.
/// Usage:
///     try!(ensure_argcount("function", min, max));
///
fn ensure_argcount(name: &str, args: &Args, min: usize, max: usize) -> Result<(), Error> {
    let count = args.len();
    if min <= count && count <= max {
        return Ok(());
    }

    let expected = if min == max { format!("{}", min) }
                   else { format!("{} to {}", min, max) };
    Err(Error::new(&format!(
        "invalid number of arguments to {}(): expected {}, got {}", name, expected, count
    )))
}
