//! Evaluation context.

use super::functions::{Args, Functions};
use super::value::Value;
use super::variables::Variables;


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
        self.vars.set(name, value)
    }

    /// Resolve given value, producing either it's copy
    /// or a value of a variable it's referring to.
    pub fn resolve(&self, value: &Value) -> Value {
        self.vars.resolve(value)
    }

    /// Call a function of given name with given arguments.
    /// Returns Some(result), or None if the function couldn't be found.
    pub fn call_func(&self, name: &str, args: Args) -> super::Result {
        self.funcs.call(name, args)
    }
}
