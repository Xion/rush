//! Evaluation context.

use std::collections::HashMap;

use rand;

use super::value::Value;


/// Evaluation context for an expression.
///
/// Contains all the variable and function bindings that are used
/// when evaluation an expression.
pub struct Context {
    vars: Variables,
    funcs: Functions,
}

impl Context {
    pub fn new() -> Context {
        Context{vars: Variables::new(), funcs: Functions::new()}
    }

    /// Retrieves the value for a variable if it exists.
    pub fn get_var(&self, name: &str) -> Option<&Value> {
        self.vars.get(&name.to_string())
    }

    /// Set a value for a variable.
    /// If the variable didn't exist before, it is created.
    pub fn set_var(&mut self, name: &str, value: Value) {
        let name = name.to_string();
        if let Some(val) = self.vars.get_mut(&name) {
            *val = value;
            return
        }
        self.vars.insert(name, value);
    }

    /// Set a string value for a variable.
    pub fn set_str_var(&mut self, name: &str, value: &str) {
        self.set_var(name, Value::String(value.to_string()))
    }

    /// Call a function of given name with given arguments.
    /// Returns Some(result), or None if the function couldn't be found.
    pub fn call_func(&self, name: &str, args: Args) -> Option<Value> {
        self.funcs.call(name, args)
    }
}


/// Container of functions available within the evaluation context.
struct Functions {
    funcs: HashMap<String, Box<Function>>,
}

impl Functions {
    pub fn new() -> Functions {
        let mut fs = Functions{funcs: HashMap::new()};

        fs.define_nullary("rand", Box::new(|| {
            Some(Value::Float(rand::random::<f64>()))
        }));

        fs.define_unary("rev", Box::new(|value| {
            value.map_str(|s: &str| {
                // TODO(xion): since this reverses chars not graphemes,
                // it mangles some non-Latin strings;
                // fix with unicode-segmentation crate
                s.chars().rev().collect::<String>()
            })
        }));
        fs.define_unary("abs", Box::new(|value| {
            match value {
                v @ Value::Integer(_) => v.map_int(i64::abs),
                v @ Value::Float(_) => v.map_float(f64::abs),
                _ => None,
            }
        }));
        fs.define_unary("len", Box::new(|value| {
            match value {
                Value::String(ref s) => Some(Value::Integer(s.len() as i64)),
                _ => None,
            }
        }));
        fs.define_unary("str", Box::new(|value| value.to_string_value()));
        fs.define_unary("int", Box::new(|value| value.to_int_value()));

        fs.define_binary("at", Box::new(|idx, value| {
            match (idx, value) {
                (Value::Integer(i),
                 Value::String(s)) => s.chars().nth(i as usize).map(|c| {
                    let mut result = String::new();
                    result.push(c);
                    Value::String(result)
                }),
                _ => None,
            }
        }));

        return fs;
    }

    pub fn call(&self, name: &str, args: Args) -> Option<Value>  {
        self.funcs.get(&name.to_string()).map(|func| func(args))
    }

    fn define(&mut self, name: &str,
              func: Box<Fn(Args) -> Option<Value>>) -> &mut Self {
        let name = name.to_string();
        self.funcs.insert(name.to_owned(), Box::new(move |args: Args| {
            // TODO(xion): better error messages for different problems;
            // for example, we could remember the arity of functions
            // and say "too many/few arguments"
            func(args).expect(&format!("invalid arguments to {}()", name))
        }));
        self
    }

    fn define_nullary(&mut self, name: &str,
                      func: Box<Fn() -> Option<Value>>) -> &mut Self {
        self.define(name, Box::new(move |args: Args| {
            if args.len() == 0 {
                func()
            } else {
                None
            }
        }))
    }

    fn define_unary(&mut self, name: &str,
                    func: Box<Fn(Value) -> Option<Value>>) -> &mut Self {
        self.define(name, Box::new(move |args: Args| {
            if args.len() == 1 {
                func(args[0].clone())
            } else {
                None
            }
        }))
    }

    fn define_binary(&mut self, name: &str,
                     func: Box<Fn(Value, Value) -> Option<Value>>) -> &mut Self {
        self.define(name, Box::new(move |args: Args| {
            if args.len() == 2 {
                func(args[0].clone(), args[1].clone())
            } else {
                None
            }
        }))
    }
}

// Type aliases to make working with functions easier.
type Args = Vec<Value>;
type Function = Fn(Args) -> Value;


/// Type for a container of variables within a scope.
type Variables = HashMap<String, Value>;
