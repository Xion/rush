//! Module implementing the AST node that represents an array subscript operation,
//! i.e. an expression in the form of `x[i]`.

use std::fmt;

use eval::{self, Eval, EvalResult, Context, Value};


/// Represents an operation of taking a subscript of an object ("indexing").
/// The object is commonly an array or a string.
pub struct SubscriptNode {
    pub object: Box<Eval>,
    pub index: Box<Eval>,
}


impl fmt::Debug for SubscriptNode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "<Index: {:?}[{:?}]>", self.object, self.index)
    }
}


impl Eval for SubscriptNode {
    fn eval(&self, context: &Context) -> EvalResult {
        let object = try!(self.object.eval(&context));
        let index = try!(self.index.eval(&context));

        match object {
            Value::Array(ref a) => SubscriptNode::eval_on_array(&a, &index),
            Value::String(ref s) => SubscriptNode::eval_on_string(&s, &index),
            _ => Err(eval::Error::new(
                &format!("can't index {:?} with {:?}", object, index)
            )),
        }
    }
}


impl SubscriptNode {
    // TODO(xion): consider supporting Python-style negative indices

    fn eval_on_array(array: &Vec<Value>, index: &Value) -> EvalResult {
        match *index {
            Value::Integer(i) => {
                if i < 0 {
                    return Err(eval::Error::new(
                        &format!("array index cannot be negative; got {}", i)
                    ));
                }
                let idx = i as usize;
                if idx >= array.len() {
                    return Err(eval::Error::new(
                        &format!("array index out of range ({})", i)
                    ));
                }
                // TODO(xion): the clone below is very inefficient for
                // multi-dimensional arrays; return some Value pointer instead
                Ok(array[idx].clone())
            },
            Value::Float(..) => Err(eval::Error::new(
                &format!("array indices must be integers")
            )),
            _ => Err(eval::Error::new(
                &format!("can't index an array with {:?}", index)
            )),
        }
    }

    fn eval_on_string(string: &String, index: &Value) -> EvalResult {
        match *index {
            Value::Integer(i) => {
                string.chars().nth(i as usize)
                    .ok_or_else(|| eval::Error::new(
                        &format!("character index out of range: {}", i)
                    ))
                    .map(|c| {
                        let mut result = String::new();
                        result.push(c);
                        Value::String(result)
                    })
            },
            _ => Err(eval::Error::new(
                &format!("can't index a string with {:?}", index)
            )),
        }
    }
}
