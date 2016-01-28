//! Evaluation context.

use std::collections::HashMap;

use super::functions::{Args, Functions};
use super::value::Value;


/// Type for a container of variables within a scope.
type Variables = HashMap<String, Value>;


/// Evaluation context for an expression.
///
/// Contains all the variable and function bindings that are used
/// when evaluating an expression.
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
