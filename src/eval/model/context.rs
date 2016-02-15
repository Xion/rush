//! Evaluation context.

use eval;
use super::functions::{Args, Functions};
use super::value::Value;
use super::variables::Variables;


/// Evaluation context for an expression.
///
/// Contains all the variable and function bindings that are used
/// when evaluating an expression.
pub struct Context<'a> {
    parent: Option<&'a Context<'a>>,
    vars: Variables,
    funcs: Functions,
}

impl<'a> Context<'a> {
    pub fn new() -> Context<'a> {
        Context{parent: None, vars: Variables::new(), funcs: Functions::new()}
    }
    pub fn with_parent(parent: &'a Context<'a>) -> Context<'a> {
        Context{
            parent: Some(parent), vars: Variables::new(), funcs: Functions::new()
        }
    }

    #[allow(dead_code)]
    /// Retrieves the value for a variable if it exists.
    pub fn get_var(&self, name: &str) -> Option<&Value> {
        self.vars.get(&name.to_string())
            .or_else(|| self.parent.and_then(|ctx| ctx.get_var(name)))
    }

    /// Set a value for a variable.
    /// If the variable didn't exist before, it is created.
    pub fn set_var(&mut self, name: &str, value: Value) {
        self.vars.set(name, value)
    }

    /// Resolve given value, producing either it's copy
    /// or a value of a variable it's referring to.
    pub fn resolve(&self, value: &Value) -> Value {
        let value = self.vars.resolve(value);
        match self.parent {
            Some(ctx) => ctx.resolve(&value),
            _ => value,
        }
    }

    /// Call a function of given name with given arguments.
    /// Returns Some(result), or None if the function couldn't be found.
    pub fn call_func(&self, name: &str, args: Args) -> eval::Result {
        // TODO(xion): if function can't be found in current context,
        // try the parent (merge var & func namespaces first, though)
        self.funcs.call(name, args)
    }
}
