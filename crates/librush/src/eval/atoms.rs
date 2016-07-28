//! Module implementing evaluation of the "atomic" expressions,
//! i.e. those that create the values that are then operated upon.

use eval::{self, Context, Eval, Value};
use eval::model::value::{ArrayRepr, ObjectRepr};
use parse::ast::{ArrayNode, ObjectNode, ScalarNode};


/// Evaluate the AST node representing a scalar value.
impl Eval for ScalarNode {
    #[inline]
    fn eval(&self, context: &mut Context) -> eval::Result {
        Ok(context.resolve(&self.value))
    }
}


/// Evaluate the AST node representing an array value.
impl Eval for ArrayNode {
    fn eval(&self, context: &mut Context) -> eval::Result {
        let mut elems = ArrayRepr::with_capacity(self.elements.len());
        for ref x in &self.elements {
            let elem = try!(x.eval(context));
            elems.push(elem);
        }
        Ok(Value::Array(elems))
    }
}


/// Evaluate the AST node representing an object value.
impl Eval for ObjectNode {
    fn eval(&self, context: &mut Context) -> eval::Result {
        let mut attrs = ObjectRepr::with_capacity(self.attributes.len());
        for &(ref k, ref v) in &self.attributes {
            let key = try!(k.eval(context));
            let value = try!(v.eval(context));
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
