//! Module implementing evaluation of the "atomic" expressions,
//! i.e. those that create the values that are then operated upon.

use std::collections::HashMap;

use eval::{self, Context, Eval, Value};
use parse::ast::{ArrayNode, ObjectNode, ScalarNode};


/// Evaluate the AST node representing a scalar value.
impl Eval for ScalarNode {
    fn eval(&self, context: &Context) -> eval::Result {
        Ok(context.resolve(&self.value))
    }
}


/// Evaluate the AST node representing an array value.
impl Eval for ArrayNode {
    fn eval(&self, context: &Context) -> eval::Result {
        // evaluate all the elements first and bail if any of that fails
        // TODO(xion): like the evaluation of objects, make this lazy
        // (currently, all items are evaluated, even if others already failed)
        let evals: Vec<_> =
            self.elements.iter().map(|x| x.eval(&context)).collect();
        if let Some(res) = evals.iter().find(|r| r.is_err()) {
            return res.clone();
        }

        // extract the element values and create the array
        let elems = evals.into_iter().map(|r| r.ok().unwrap()).collect();
        Ok(Value::Array(elems))
    }
}


/// Evaluate the AST node representing an object value.
impl Eval for ObjectNode {
    fn eval(&self, context: &Context) -> eval::Result {
        let mut attrs: HashMap<String, Value> = HashMap::new();
        for &(ref k, ref v) in self.attributes.iter() {
            let key = try!(k.eval(&context));
            let value = try!(v.eval(&context));
            if let Value::String(attr) = key {
                attrs.insert(attr, value);
            } else {
                return Err(eval::Error::new(&format!(
                    "object attribute name must be string, got {}", key.typename()
                )));
            }
        }
        Ok(Value::Object(attrs))
    }
}
