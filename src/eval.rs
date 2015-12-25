//! Module implementing evaluation of parsed expressions.

use std::collections::HashMap;


// TODO(xion): make an ADT or something, to represent various types of values
pub type Value = String;


/// Evaluation context for an expression.
///
/// Contains all the variable and function bindings that are used
/// when evaluation an expression.
pub struct Context {
    vars: HashMap<String, Value>,
}

impl Context {
    pub fn new() -> Context {
        Context{vars: HashMap::new()}
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
            *val = name;
            return
        }
        self.vars.insert(name, value);
    }
}


/// Trait for objects that can be evaluated within given Context.
pub trait Eval {
    fn eval(&self, context: &Context) -> Value;
}
