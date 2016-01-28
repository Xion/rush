/// Functions available to the expressions.

use std::collections::HashMap;

use rand;

use super::value::Value;


/// Arguments to a function.
pub type Args = Vec<Value>;

/// Function type.
pub type Function = Fn(Args) -> Value;


/// Container of functions available within the evaluation context.
pub struct Functions {
    funcs: HashMap<String, Box<Function>>,
}

impl Functions {
    pub fn new() -> Functions {
        let mut fs = Functions{funcs: HashMap::new()};

        fs.define_nullary("rand", || Some(Value::Float(rand::random::<f64>())));

        fs.define_unary("rev", |value| {
            value.map_str(|s: &str| {
                // TODO(xion): since this reverses chars not graphemes,
                // it mangles some non-Latin strings;
                // fix with unicode-segmentation crate
                s.chars().rev().collect::<String>()
            })
        });
        fs.define_unary("abs", |value| {
            match value {
                v @ Value::Integer(_) => v.map_int(i64::abs),
                v @ Value::Float(_) => v.map_float(f64::abs),
                _ => None,
            }
        });
        fs.define_unary("len", |value| {
            match value {
                Value::String(ref s) => Some(Value::Integer(s.len() as i64)),
                Value::Array(ref a) => Some(Value::Integer(a.len() as i64)),
                _ => None,
            }
        });
        fs.define_unary("str", |value| value.to_string_value());
        fs.define_unary("int", |value| value.to_int_value());
        fs.define_unary("float", |value| value.to_float_value());
        fs.define_unary("bool", |value| value.to_bool_value());

        fs.define_binary("split", |string, delim| {
            if let (Value::String(ref s), Value::String(ref d)) = (string, delim) {
                let segments: Vec<_> = s.split(d)
                    .map(str::to_owned).map(Value::String)
                    .collect();
                return Some(Value::Array(segments));
            }
            None
        });
        fs.define_binary("join", |array, delim| {
            if let (Value::Array(ref a), Value::String(ref d)) = (array, delim) {
                let strings: Vec<_> =  a.iter()
                    .map(Value::to_string_value).filter(Option::is_some)
                    .map(Option::unwrap).map(Value::unwrap_string)
                    .collect();
                if strings.len() == a.len() {
                    return Some(Value::String(strings.join(&d)));
                }
            }
            None
        });

        // TODO(xion): allow this function to accept just two arguments,
        // with the third one being an implicit reference to the default var
        // (requires allowing functions to access the Context)
        fs.define_ternary("sub", |needle, replacement, haystack| {
            if let (Value::String(n),
                    Value::String(r),
                    Value::String(h)) = (needle, replacement, haystack) {
                return Some(Value::String(h.replace(&n, &r)));
            }
            None
        });

        return fs;
    }

    pub fn call(&self, name: &str, args: Args) -> Option<Value>  {
        self.funcs.get(&name.to_string()).map(|func| func(args))
    }

    fn define<F>(&mut self, name: &str, func: F) -> &mut Self
        where F: Fn(Args) -> Option<Value> + 'static
    {
        let name = name.to_string();
        self.funcs.insert(name.to_owned(), Box::new(move |args: Args| {
            // TODO(xion): better error messages for different problems;
            // for example, we could remember the arity of functions
            // and say "too many/few arguments"
            func(args).expect(&format!("invalid arguments to {}()", name))
        }));
        self
    }

    fn define_nullary<F>(&mut self, name: &str, func: F) -> &mut Self
        where F: Fn() -> Option<Value> + 'static
    {
        self.define(name, move |args: Args| {
            if args.len() == 0 {
                func()
            } else {
                None
            }
        })
    }

    fn define_unary<F>(&mut self, name: &str, func: F) -> &mut Self
        where F: Fn(Value) -> Option<Value> + 'static
    {
        self.define(name, move |args: Args| {
            if args.len() == 1 {
                func(args[0].clone())
            } else {
                None
            }
        })
    }

    fn define_binary<F>(&mut self, name: &str, func: F) -> &mut Self
        where F: Fn(Value, Value) -> Option<Value> + 'static
    {
        self.define(name, move |args: Args| {
            if args.len() == 2 {
                func(args[0].clone(), args[1].clone())
            } else {
                None
            }
        })
    }

    fn define_ternary<F>(&mut self, name: &str, func: F) -> &mut Self
        where F: Fn(Value, Value, Value) -> Option<Value> + 'static
    {
        self.define(name, move |args: Args| {
            if args.len() == 3 {
                func(args[0].clone(), args[1].clone(), args[2].clone())
            } else {
                None
            }
        })
    }
}
