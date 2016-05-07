//! Module implementing evaluation of block AST nodes.

use eval::{self, Context, Value, Eval};
use parse::ast::BlockNode;


/// Evaluate the AST node representing a block of expressions.
impl Eval for BlockNode {
    fn eval(&self, context: &mut Context) -> eval::Result {
        let mut result = Value::Empty;
        for expr in &self.expressions {
            result = try!(expr.eval(context));
        }
        Ok(result)
    }
}
