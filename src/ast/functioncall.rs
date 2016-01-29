//! Module implementing AST node that represents a function call.

use std::fmt;

use eval::{self, Eval, Context};


/// Represents a call to a function with given name and arguments.
///
/// The exact function the name resolves to depends on the context passed
/// during evaluation.
pub struct FunctionCallNode {
    pub name: String,
    pub args: Vec<Box<Eval>>,
}


impl fmt::Debug for FunctionCallNode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "<Call: {}({})>", self.name,
               self.args.iter()
                   .map(|ref arg| format!("{:?}", arg))
                   .collect::<Vec<String>>().join(","))
    }
}


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
