//! Module implementing the evaluation of "trailer" parts of terms,
//! such as indexing or function call syntax.

use eval::{self, api, Context, Eval, Value};
use eval::model::Invoke;
use eval::model::value::{ArrayRepr, ObjectRepr, StringRepr};
use parse::ast::{FunctionCallNode, SubscriptNode};


/// Evaluate the function call AST node.
impl Eval for FunctionCallNode {
    fn eval(&self, context: &Context) -> eval::Result {
        let func = try!(self.func.eval(&context));
        let func_type = func.typename();

        if let Value::Function(mut f) = func {
            // evaluate all the arguments first, bail if any of that fails
            let evals: Vec<_> = self.args.iter()
                .map(|opt_arg| opt_arg.as_ref()
                    .expect("currying with non-initial arguments is NYI"))
                .map(|arg| arg.eval(&context))
                .collect();
            if let Some(res) = evals.iter().find(|r| r.is_err()) {
                return res.clone();
            }

            // extract the argument values and determine
            // if it's a regular call or a curry (partial application)
            let args: Vec<_> =
                evals.into_iter().map(|r| r.ok().unwrap()).collect();
            if f.arity() > args.len() {
                for arg in args.into_iter() {
                    f = f.curry(arg).unwrap();
                }
                return Ok(Value::Function(f));
            } else {
                return f.invoke(args, &context);
            }
        }

        Err(eval::Error::new(&format!(
            "can't call a(n) {} like a function", func_type
        )))
    }
}


/// Evaluate the subscript AST node.
impl Eval for SubscriptNode {
    fn eval(&self, context: &Context) -> eval::Result {
        let object = try!(self.object.eval(&context));
        let index = try!(self.index.eval(&context));

        // TODO(xion): roll this into eval_on_array(), which would require
        // copying parts of the filter() function implementation
        if object.is_array() && index.is_function() {
            return api::base::filter(index, object, &context);
        }

        match object {
            Value::String(ref s) => SubscriptNode::eval_on_string(&s, &index),
            Value::Array(ref a) => SubscriptNode::eval_on_array(&a, &index),
            Value::Object(ref o) => SubscriptNode::eval_on_object(&o, &index),
            _ => Err(eval::Error::new(
                &format!("can't index {:?} with {:?}", object, index)
            )),
        }
    }
}

impl SubscriptNode {
    fn eval_on_string(string: &StringRepr, index: &Value) -> eval::Result {
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
                &format!("can't index a string with a {}", index.typename())
            )),
        }
    }

    fn eval_on_array(array: &ArrayRepr, index: &Value) -> eval::Result {
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
                &format!("can't index an array with a {}", index.typename())
            )),
        }
    }

    fn eval_on_object(object: &ObjectRepr, index: &Value) -> eval::Result {
        match *index {
            Value::Symbol(ref s) |
            Value::String(ref s) => object.get(s)
                .map(Value::clone)  // TODO(xion): same as in eval_on_array()
                .ok_or_else(|| eval::Error::new(&format!(
                    "object has no attribute `{}`", s
                ))),
            _ => Err(eval::Error::new(
                &format!("can't index an object with a {}", index.typename())
            ))
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
