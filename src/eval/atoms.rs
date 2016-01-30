//! Module implementing evaluation of the "atomic" expressions,
//! i.e. those that create the values that are then operated upon.

use eval::{self, Context, Eval, Value};
use parse::ast::{ArrayNode, AtomNode};


/// Evaluate the AST node representing a scalar value.
impl Eval for AtomNode {
    fn eval(&self, context: &Context) -> eval::Result {
        Ok(context.resolve(&self.value))
    }
}


/// Evaluate the AST node representing an array value.
impl Eval for ArrayNode {
    fn eval(&self, context: &Context) -> eval::Result {
        // evaluate all the elements first and bail if any of that fails
        let evals: Vec<_> =
            self.elements.iter().map(|x| x.eval(&context)).collect();
        if let Some(res) = evals.iter().find(|r| r.is_err()) {
            return res.clone();
        }

        // extract the element values and create the array
        let elems = evals.iter().map(|r| r.clone().ok().unwrap()).collect();
        Ok(Value::Array(elems))
    }
}
