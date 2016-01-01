//! Module implementing AST node that represents a function call.

use eval::{self, Eval, EvalResult, Context};


/// Represents a call to a function with given name and arguments.
///
/// The exact function the name resolves to depends on the context passed
/// during evaluation.
pub struct FunctionCallNode {
    pub name: String,
    pub args: Vec<Box<Eval>>,
}


impl Eval for FunctionCallNode {
    fn eval(&self, context: &Context) -> EvalResult {
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
