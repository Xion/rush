//! Module implementing data structures for holding functions
//! that available to the expressions.

use std::collections::HashMap;
use std::ptr;

use eval::{self, api, Error};
use super::value::Value;


/// Arguments to a function.
pub type Args = Vec<Value>;

/// Function type.
pub type Function = Fn(Args) -> eval::Result;


/// Container of functions available within the evaluation context.
pub struct Functions {
    funcs: HashMap<String, Box<Function>>,
}

impl Functions {
    pub fn new() -> Functions {
        let mut fs = Functions{funcs: HashMap::new()};

        fs.define_nullary("rand", api::rand);

        fs.define_unary("rev", api::rev);
        fs.define_unary("abs", api::abs);
        fs.define_unary("len", api::len);
        fs.define_unary("str", api::str);
        fs.define_unary("int", api::int);
        fs.define_unary("float", api::float);
        fs.define_unary("bool", api::bool);

        fs.define_binary("split", api::split);
        fs.define_binary("join", api::join);

        fs.define_ternary("sub", api::sub);

        return fs;
    }

    pub fn call(&self, name: &str, args: Args) -> eval::Result  {
        self.funcs.get(&name.to_string())
            .ok_or(Error::new(&format!("{}() function not found", name)))
            .and_then(|func| func(args))
    }

    fn define<F>(&mut self, name: &str, func: F) -> &mut Self
        where F: Fn(Args) -> eval::Result + 'static
    {
        self.funcs.insert(name.to_owned(), Box::new(func));
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

    // The `unsafe` blocks in the functions below are in fact
    // perfectly safe, thanks to ensure_argcount() and the fact that we're
    // not using the `args` for anything else after calling `func`.

    fn define_unary<F>(&mut self, name: &'static str, func: F) -> &mut Self
        where F: Fn(Value) -> eval::Result + 'static
    {
        self.define(name, move |args: Args| {
            try!(ensure_argcount(name, &args, 1, 1));
            unsafe {
                let args = args.as_ptr();
                func(ptr::read(args.offset(0)))
            }
        })
    }

    fn define_binary<F>(&mut self, name: &'static str, func: F) -> &mut Self
        where F: Fn(Value, Value) -> eval::Result + 'static
    {
        self.define(name, move |args: Args| {
            try!(ensure_argcount(name, &args, 2, 2));
            unsafe {
                let args = args.as_ptr();
                func(ptr::read(args.offset(0)), ptr::read(args.offset(1)))
            }
        })
    }

    fn define_ternary<F>(&mut self, name: &'static str, func: F) -> &mut Self
        where F: Fn(Value, Value, Value) -> eval::Result + 'static
    {
        self.define(name, move |args: Args| {
            try!(ensure_argcount(name, &args, 3, 3));
            unsafe {
                let args = args.as_ptr();
                func(ptr::read(args.offset(0)),
                     ptr::read(args.offset(1)),
                     ptr::read(args.offset(2)))
            }
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
