/// Module implementing data structures for holding variables
/// that are available to -- and can be modified by -- the evaluated expressions.

use std::collections::HashMap;

use super::value::Value;


/// Container for variables available within the evaluation context.
pub struct Variables {
    vars: HashMap<String, Value>
}

impl Variables {
    pub fn new() -> Variables {
        Variables{vars: HashMap::new()}
    }

    /// Check whether variable with a given name exists.
    pub fn exists(&self, name: &str) -> bool {
        self.vars.contains_key(name)
    }

    /// Retrieves the value for a variable if it exists.
    pub fn get(&self, name: &str) -> Option<&Value> {
        self.vars.get(name)
    }

    /// Set a value for a variable.
    /// If the variable didn't exist before, it is created.
    pub fn set(&mut self, name: &str, value: Value) {
        let name = name.to_string();
        if let Some(val) = self.vars.get_mut(&name) {
            *val = value;
            return
        }
        self.vars.insert(name, value);
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
    // TODO(xion): when functions are first-class values,
    // the above method may either need an equivalent in Functions,
    // or be moved to Context after namespaces for vars & funcs are merged
}
