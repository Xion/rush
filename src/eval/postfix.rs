//! Module implementing the evaluation of postfix operators.

use eval::{self, Context, Eval, Value};
use parse::ast::{FunctionCallNode, SubscriptNode};


/// Evaluate the function call AST node.
impl Eval for FunctionCallNode {
    fn eval(&self, context: &Context) -> eval::Result {
        // evaluate all the arguments first, bail if any of that fails
        let evals: Vec<_> =
            self.args.iter().map(|x| x.eval(&context)).collect();
        if let Some(res) = evals.iter().find(|r| r.is_err()) {
            return res.clone();
        }

        // extract the argument values and call the function
        let args = evals.iter().map(|r| r.clone().ok().unwrap()).collect();
        context.call_func(&self.name, args)
    }
}


/// Evaluate the array subscripting AST node.
impl Eval for SubscriptNode {
    fn eval(&self, context: &Context) -> eval::Result {
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
    fn eval_on_array(array: &Vec<Value>, index: &Value) -> eval::Result {
        match *index {
            Value::Integer(i) => {
                SubscriptNode::resolve_index(i as isize, array.len()).map(|idx| {
                    // TODO(xion): this clone() call is very inefficient for
                    // multi-dimensional arrays; return some Value pointer instead
                    array[idx].clone()
                })
            },
            Value::Float(..) => Err(eval::Error::new(
                &format!("array indices must be integers")
            )),
            _ => Err(eval::Error::new(
                &format!("can't index an array with {:?}", index)
            )),
        }
    }

    fn eval_on_string(string: &String, index: &Value) -> eval::Result {
        match *index {
            Value::Integer(i) => {
                SubscriptNode::resolve_index(i as isize, string.len()).map(|idx| {
                    let c = string.chars().nth(idx).unwrap();
                    let mut result = String::new();
                    result.push(c);
                    Value::String(result)
                })
            },
            Value::Float(..) => Err(eval::Error::new(
                &format!("character indices must be integers")
            )),
            _ => Err(eval::Error::new(
                &format!("can't index a string with {:?}", index)
            )),
        }
    }

    /// Resolve index against the total length of a sequence.
    /// If negative, it will be interpreted as counting from the end.
    fn resolve_index(index: isize, len: usize) -> Result<usize, eval::Error> {
        if index >= 0 {
            let index = index as usize;
            if index >= len {
                Err(eval::Error::new(&format!("index out of range ({})", index)))
            } else {
                Ok(index as usize)
            }
        } else {
            let index = (-index) as usize;
            if index > len {
                Err(eval::Error::new(&format!("index out of range (-{})", index)))
            } else {
                Ok(len - index)
            }
        }
    }
}
