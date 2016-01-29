//! Module implementing the AST node that represents an expression for creating an array.

use std::fmt;

use eval::{self, Eval, Context, Value};


pub struct ArrayNode {
    pub elements: Vec<Box<Eval>>
}


impl fmt::Debug for ArrayNode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "<Array: [{}]>", self.elements.iter()
            .map(|ref elem| format!("{:?}", elem))
            .collect::<Vec<String>>().join(","))
    }
}


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
