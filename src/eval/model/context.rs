//! Evaluation context.

use std::borrow::{Borrow, ToOwned};
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::{BuildHasherDefault, Hash};

use fnv::FnvHasher;

use eval;
use super::{Args, Invoke, Value};


/// Type for names of variables present in the Context.
pub type Name = String;

/// Custom hasher for hashmap type that stores variables present in Context.
/// Uses the Fowler-Noll-Vo hashing algorithm which is faster for short keys.
type Hasher = BuildHasherDefault<FnvHasher>;


/// Evaluation context for an expression.
///
/// Contains all the variable and function bindings that are used
/// when evaluating an expression.
///
/// This is roughly equivalent to a stack frame,
/// or a block of code in languages with local scoping (like C++ or Rust).
pub struct Context<'c> {
    /// Optional parent Context, i.e. a lower "frame" on the "stack".
    parent: Option<&'c Context<'c>>,

    /// Names & values present in the context.
    scope: HashMap<Name, Value, Hasher>,
}

impl<'c> Context<'c> {
    /// Create a new root context.
    pub fn new() -> Context<'c> {
        let mut context = Context{parent: None, scope: HashMap::default()};
        context.init_builtins();
        context
    }

    /// Create a new Context that's a child of given parent.
    #[inline(always)]
    pub fn with_parent(parent: &'c Context<'c>) -> Context<'c> {
        Context{parent: Some(parent), scope: HashMap::default()}
    }

    /// Whether this is a root context (one without a parent).
    #[inline(always)]
    pub fn is_root(&self) -> bool {
        self.parent.is_none()
    }

    /// Check if given name is defined within this Context
    /// or any of its ancestors.
    #[inline]
    pub fn is_defined<N: ?Sized>(&self, name: &N) -> bool
        where Name: Borrow<N>, N: Hash + Eq
    {
        self.scope.get(name)
            .or_else(|| self.parent.and_then(|ctx| ctx.get(name)))
            .is_some()
    }

    /// Check if given name is defined in this context.
    /// Does not look at parent Contexts.
    #[inline(always)]
    pub fn is_defined_here<N: ?Sized>(&self, name: &N) -> bool
        where Name: Borrow<N>, N: Hash + Eq
    {
        self.scope.get(name).is_some()
    }

    /// Retrieves a value by name from the scope of the context
    /// or any of its parents.
    #[inline]
    pub fn get<N: ?Sized>(&self, name: &N) -> Option<&Value>
        where Name: Borrow<N>, N: Hash + Eq
    {
        self.scope.get(name)
            .or_else(|| self.parent.and_then(|ctx| ctx.get(name)))
    }

    /// Set a value for a variable inside the context's scope.
    /// If the name already exists in the parent scope (if any),
    /// it will be shadowed.
    #[inline(always)]
    pub fn set<N: ?Sized>(&mut self, name: &N, value: Value)
        where Name: Borrow<N>, N: ToOwned<Owned=Name>
    {
        self.scope.insert(name.to_owned(), value);
    }

    /// "Unset" the value of a variable, making the symbol undefined
    /// in this context.
    ///
    /// Returns a boolean indicating whether the context changed
    /// (i.e. the variable was actually defined before).
    ///
    /// Note how regardless of the return value, the variable won't be defined
    /// in this context after the call to this method. It may, however,
    /// still **be** defined in a parent Context, if any.
    pub fn unset_here<N: ?Sized>(&mut self, name: &N) -> bool
        where Name: Borrow<N>, N: Hash + Eq
    {
        self.scope.remove(name).is_some()
    }

    /// Reset the context, removing all variable bindings.
    /// Built-in functions and constants are preserved.
    pub fn reset(&mut self) {
        self.scope.clear();
        if self.is_root() {
            self.init_builtins();
        }
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
    pub fn call<N: ?Sized>(&self, name: &N, args: Args) -> eval::Result
        where Name: Borrow<N>, N: Hash + Eq + Display
    {
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
