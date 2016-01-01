//! Data structures representing the abstract syntax tree (AST)
//! of parsed expressions.

mod binaryop;
mod unaryop;

pub use self::binaryop::*;
pub use self::unaryop::*;


use std::str::FromStr;

use eval::{self, Eval, EvalResult, Context, Value};


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


pub struct FunctionCallNode {
    pub name: String,
    pub args: Vec<Box<Eval>>,
}

impl Eval for FunctionCallNode {
    fn eval(&self, context: &Context) -> Result<Value, eval::Error> {
        // evaluate all the arguments first, bail if any of that fails
        let evals: Vec<_> =
            self.args.iter().map(|x| x.eval(&context)).collect();
        if let Some(res) = evals.iter().find(|r| r.is_err()) {
            return res.clone();
        }

        // extract the argument values and call the function
        let args = evals.iter().map(|r| r.clone().ok().unwrap()).collect();
        context.call_func(&self.name, args).ok_or(
            eval::Error::new(&format!("unknown function: {}", self.name)))
    }
}
