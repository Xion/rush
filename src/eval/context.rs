//! Evaluation context.

use std::collections::HashMap;

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

        return fs;
    }

    pub fn call(&self, name: &str, args: Args) -> Option<Value>  {
        self.funcs.get(&name.to_string()).map(|func| func(args))
    }

    fn define(&mut self, name: &str,
              func: Box<Fn(Args) -> Option<Value>>) -> &mut Self {
        let name = name.to_string();
        self.funcs.insert(name.clone(), Box::new(move |args: Args| {
            func(args).expect(&format!("invalid arguments to {}()", name))
        }));
        self
    }

    fn define_unary(&mut self, name: &str,
                    func: Box<Fn(Value) -> Option<Value>>) -> &mut Self {
        self.define(name, Box::new(
            move |args: Args| func(args[0].clone())))
    }

    fn define_binary(&mut self, name: &str,
                     func: Box<Fn(Value, Value) -> Option<Value>>) -> &mut Self {
        self.define(name, Box::new(
            move |args: Args| func(args[0].clone(), args[1].clone())))
    }
}

// Type aliases to make working with functions easier.
type Args = Vec<Value>;
type Function = Fn(Args) -> Value;


/// Type for a container of variables within a scope.
type Variables = HashMap<String, Value>;
