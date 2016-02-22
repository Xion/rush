//! Evaluation context.

use std::collections::HashMap;

use eval;
use super::function::{Args, Invoke};
use super::value::Value;


/// Evaluation context for an expression.
///
/// Contains all the variable and function bindings that are used
/// when evaluating an expression.
///
/// This is roughly equivalent to a stack frame.
pub struct Context<'a> {
    /// Optional parent Context, i.e. a lower "frame" on the "stack".
    parent: Option<&'a Context<'a>>,

    /// Names & values present in the context.
    scope: HashMap<String, Value>,
}

impl<'a> Context<'a> {
    /// Create a new root context.
    pub fn new() -> Context<'a> {
        let mut context = Context{parent: None, scope: HashMap::new()};
        context.init_builtins();
        context
    }

    /// Create a new Context that's a child of given parent.
    pub fn with_parent(parent: &'a Context<'a>) -> Context<'a> {
        Context{parent: Some(parent), scope: HashMap::new()}
    }

    /// Whether this is a root context (one without a parent).
    pub fn is_root(&self) -> bool {
        self.parent.is_none()
    }

    /// Check if given name is defined within this Context
    /// or any of its ancestors.
    pub fn is_defined(&self, name: &str) -> bool {
        self.scope.get(name)
            .or_else(|| self.parent.and_then(|ctx| ctx.get(name)))
            .is_some()
    }

    /// Check if given name is defined in this context.
    /// Does not look at parent Contexts.
    pub fn is_defined_here(&self, name: &str) -> bool {
        self.scope.get(name).is_some()
    }

    /// Retrieves a value by name from the scope of the context
    /// or any of its parents.
    pub fn get(&self, name: &str) -> Option<&Value> {
        self.scope.get(name)
            .or_else(|| self.parent.and_then(|ctx| ctx.get(name)))
    }

    /// Set a value for a variable inside the context's scope.
    /// If the name already exists in the parent scope (if any),
    /// it will be shadowed.
    pub fn set(&mut self, name: &str, value: Value) {
        self.scope.insert(name.to_owned(), value);
    }

    /// Resolve a possible variable reference.
    ///
    /// Returns the variable's Value (which may be just variable name as string),
    /// or a copy of the original Value if it wasn't a reference.
    pub fn resolve(&self, value: &Value) -> Value {
        let mut result = value;

        // follow the chain of references until it bottoms out
        loop {
            match result {
                &Value::Symbol(ref sym) => {
                    if let Some(target) = self.get(sym) {
                        result = target;
                    } else {
                        return Value::String(sym.clone())
                    }
                }
                _ => { break; }
            }
        }
        result.clone()
    }

    /// Call a function of given name with given arguments.
    pub fn call(&self, name: &str, args: Args) -> eval::Result {
        match self.get(name) {
            Some(&Value::Function(ref f)) => f.invoke(args, &self),
            // Note that when both this & parent context have `name` in scope,
            // and in parent this is a function while in this context it's not,
            // the result is in error.
            // This is perfectly consistent with the way shadowing should work,
            // and would be VERY confusing otherwise.
            _ => Err(eval::Error::new(&format!("`{}` is not a function", name))),
        }
    }
}
