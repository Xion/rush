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
        // TODO(xion): consider making Functions a struct and extracting
        // the boilerplate for defining functions there
        let mut funcs = Functions::new();
        funcs.insert("abs".to_string(), Box::new(|args: Vec<Value>| {
            args[0].map_float(f64::abs).expect("invalid arguments to abs()")
        }));
        funcs.insert("abs".to_string(), Box::new(|args: Vec<Value>| {
            args[0].map_int(i64::abs).expect("invalid arguments to abs()")
        }));
        funcs.insert("rev".to_string(), Box::new(|args: Vec<Value>| {
            args[0].map_str(|s: &str| {
                // TODO(xion): since this reverses chars not graphemes,
                // it mangles some non-Latin strings;
                // fix with unicode-segmentation crate
                s.chars().rev().collect::<String>()
            }).expect("invalid arguments to rev()")
        }));

        Context{vars: Variables::new(), funcs: funcs}
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
    pub fn set_string_var(&mut self, name: &str, value: &str) {
        self.set_var(name, Value::String(value.to_string()))
    }

    /// Call a function of given name with given arguments.
    /// Returns Some(result), or None if the function couldn't be found.
    pub fn call_func(&self, name: &str, args: Vec<Value>) -> Option<Value> {
        self.funcs.get(&name.to_string()).map(|func| func(args))
    }
}


/// Type for a container of functions available within the evaluation context.
type Functions = HashMap<String, Box<Fn(Vec<Value>) -> Value>>;

/// Type for a container of variables within a scope.
type Variables = HashMap<String, Value>;
