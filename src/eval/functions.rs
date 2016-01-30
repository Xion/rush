/// Module implementing data structures for holding functions
/// that available to the expressions.

use std::collections::HashMap;
use std::ptr;

use rand;

use eval;
use super::Error;
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

        fs.define_nullary("rand", || Ok(Value::Float(rand::random::<f64>())));

        fs.define_unary("rev", |value| {
            // TODO(xion): since this reverses chars not graphemes,
            // it mangles some non-Latin strings;
            // fix with unicode-segmentation crate
            eval1!(value : &String { value.chars().rev().collect() });
            Err(Error::new(&format!(
                "rev() requires a string, got {}", value.typename()
            )))
        });
        fs.define_unary("abs", |value| {
            eval1!(value : Integer { value.abs() });
            eval1!(value : Float { value.abs() });
            Err(Error::new(&format!(
                "abs() requires a number, got {}", value.typename()
            )))
        });
        fs.define_unary("len", |value| {
            eval1!((value: &String) -> Integer { value.len() as i64 });
            eval1!((value: &Array) -> Integer { value.len() as i64 });
            Err(Error::new(&format!(
                "len() requires string or array, got {}", value.typename()
            )))
        });
        fs.define_unary("str", |value| {
            value.to_string_value().ok_or_else(|| Error::new(
                &format!("cannot convert {} to string", value.typename())
            ))
        });
        fs.define_unary("int", |value| {
             value.to_int_value().ok_or_else(|| Error::new(
                &format!("cannot convert {} to int", value.typename())
            ))
        });
        fs.define_unary("float", |value| {
            value.to_float_value().ok_or_else(|| Error::new(
                &format!("cannot convert {} to float", value.typename())
            ))
        });
        fs.define_unary("bool", |value| {
            value.to_bool_value().ok_or_else(|| Error::new(
                &format!("cannot convert {} to bool", value.typename())
            ))
        });

        fs.define_binary("split", |string, delim| {
            eval2!((string: &String, delim: &String) -> Array {
                string.split(delim).map(str::to_owned).map(Value::String).collect()
            });
            Err(Error::new(&format!(
                "split() expects two strings, got: {}, {}",
                string.typename(), delim.typename()
            )))
        });
        fs.define_binary("join", |array, delim| {
            if let (&Value::Array(ref a),
                    &Value::String(ref d)) = (&array, &delim) {
                let strings: Vec<_> =  a.iter()
                    .map(Value::to_string_value).filter(Option::is_some)
                    .map(Option::unwrap).map(Value::unwrap_string)
                    .collect();
                if strings.len() == a.len() {
                    return Ok(Value::String(strings.join(&d)));
                }
            }
            Err(Error::new(&format!(
                "join() expects an array and string, got: {}, {}",
                array.typename(), delim.typename()
            )))
        });

        // TODO(xion): allow this function to accept just two arguments,
        // with the third one being an implicit reference to the default var
        // (requires allowing functions to access the Context)
        fs.define_ternary("sub", |needle, replacement, haystack| {
            if let (&Value::String(ref n),
                    &Value::String(ref r),
                    &Value::String(ref h)) = (&needle, &replacement, &haystack) {
                return Ok(Value::String(h.replace(n, r)));
            }
            Err(Error::new(&format!(
                "sub() expects three strings, got: {}, {}, {}",
                needle.typename(), replacement.typename(), haystack.typename()
            )))
        });

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
fn ensure_argcount(name: &'static str, args: &Args, min: usize, max: usize) -> Result<(), Error> {
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
