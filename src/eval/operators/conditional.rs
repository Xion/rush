//! Module implementating evaluation of conditonal operator AST nodes.

use eval::{self, api, Context, Eval};
use parse::ast::ConditionalNode;


/// Evaluate the ternary operator / conditional node.
impl Eval for ConditionalNode {
    #[inline]
    fn eval(&self, context: &mut Context) -> eval::Result {
        let condition = try!(
            self.cond.eval(context).and_then(api::conv::bool)
        ).unwrap_bool();
        if condition {
            self.then.eval(context)
        } else {
            self.else_.eval(context)
        }
    }
}
