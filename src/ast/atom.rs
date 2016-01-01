//! Module implementing the AST that represents an expression "atom".

use std::str::FromStr;

use eval::{Eval, EvalResult, Context, Value};


/// Represents the smallest, indivisible unit of an expression: a single value.
pub struct AtomNode {
    pub value: Value,
}


impl FromStr for AtomNode {
    type Err = <Value as FromStr>::Err;

    fn from_str(s: &str) -> Result<AtomNode, Self::Err> {
        s.parse::<Value>().map(|v| AtomNode{value: v})
    }
}


impl Eval for AtomNode {
    fn eval(&self, context: &Context) -> EvalResult {
        Ok(self.resolve(&context))
    }
}


impl AtomNode {
    /// Resolve a possible variable reference against given context.
    ///
    /// Returns the variable's Value (which may be just variable name as string),
    /// or a copy of the original Value if it wasn't a reference.
    fn resolve(&self, context: &Context) -> Value {
        let mut result = self.value.clone();

        // follow the chain of references until it bottoms out
        loop {
            match result {
                Value::Symbol(sym) => {
                    result = context.get_var(&sym)
                        .map(Value::clone)
                        .unwrap_or_else(move || Value::String(sym));
                }
                _ => { break; }
            }
        }
        result
    }
}
