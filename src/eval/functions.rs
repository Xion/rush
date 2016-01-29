/// Functions available to the expressions.

use std::collections::HashMap;

use rand;

use super::{EvalResult, Error};
use super::value::Value;


/// Arguments to a function.
pub type Args = Vec<Value>;

/// Function type.
pub type Function = Fn(Args) -> EvalResult;


/// Container of functions available within the evaluation context.
pub struct Functions {
    funcs: HashMap<String, Box<Function>>,
}

impl Functions {
    pub fn new() -> Functions {
        let mut fs = Functions{funcs: HashMap::new()};

        fs.define_nullary("rand", || Ok(Value::Float(rand::random::<f64>())));

        fs.define_unary("rev", |value| {
            value.map_str(|s: &str| {
                // TODO(xion): since this reverses chars not graphemes,
                // it mangles some non-Latin strings;
                // fix with unicode-segmentation crate
                s.chars().rev().collect::<String>()
            }).ok_or(Error::new(&format!("rev() requires a string")))
        });
        fs.define_unary("abs", |value| {
            match value {
                Value::Integer(i) => Ok(Value::Integer(i.abs())),
                Value::Float(f) => Ok(Value::Float(f.abs())),
                _ => Err(Error::new(&format!("abs() requires a number"))),
            }
        });
        fs.define_unary("len", |value| {
            match value {
                Value::String(ref s) => Ok(Value::Integer(s.len() as i64)),
                Value::Array(ref a) => Ok(Value::Integer(a.len() as i64)),
                _ => Err(Error::new(&format!("len() requires string or array"))),
            }
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
            if let (&Value::String(ref s),
                    &Value::String(ref d)) = (&string, &delim) {
                let segments: Vec<_> = s.split(d)
                    .map(str::to_owned).map(Value::String)
                    .collect();
                return Ok(Value::Array(segments));
            }
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
                return Ok(Value::String(h.replace(&n, &r)));
            }
            Err(Error::new(&format!(
                "sub() expects three strings, got: {}, {}, {}",
                needle.typename(), replacement.typename(), haystack.typename()
            )))
        });

        return fs;
    }

    pub fn call(&self, name: &str, args: Args) -> EvalResult  {
        self.funcs.get(&name.to_string())
            .ok_or(Error::new(&format!("{}() function not found", name)))
            .and_then(|func| func(args))
    }

    fn define<F>(&mut self, name: &str, func: F) -> &mut Self
        where F: Fn(Args) -> EvalResult + 'static
    {
        self.funcs.insert(name.to_owned(), Box::new(func));
        self
    }

    fn define_nullary<F>(&mut self, name: &'static str, func: F) -> &mut Self
        where F: Fn() -> EvalResult + 'static
    {
        self.define(name, move |args: Args| {
            try!(ensure_argcount(name, &args, 0, 0));
            func()
        })
    }

    fn define_unary<F>(&mut self, name: &'static str, func: F) -> &mut Self
        where F: Fn(Value) -> EvalResult + 'static
    {
        self.define(name, move |args: Args| {
            try!(ensure_argcount(name, &args, 1, 1));
            func(args[0].clone())
        })
    }

    fn define_binary<F>(&mut self, name: &'static str, func: F) -> &mut Self
        where F: Fn(Value, Value) -> EvalResult + 'static
    {
        self.define(name, move |args: Args| {
            try!(ensure_argcount(name, &args, 2, 2));
            func(args[0].clone(), args[1].clone())
        })
    }

    fn define_ternary<F>(&mut self, name: &'static str, func: F) -> &mut Self
        where F: Fn(Value, Value, Value) -> EvalResult + 'static
    {
        self.define(name, move |args: Args| {
            try!(ensure_argcount(name, &args, 3, 3));
            func(args[0].clone(), args[1].clone(), args[2].clone())
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
