//! Module implementing evaluation of operator-related AST nodes.
//!
//! This includes unary and conditional operators, as well as binary operators
//! and their curried versions (which evaluate into functions).

mod unary;
mod binary;
mod curried;


use eval::{self, api, Context, Eval, Value};
use parse::ast::{ConditionalNode};


/// Evaluate the ternary operator / conditional node.
impl Eval for ConditionalNode {
    #[inline]
    fn eval(&self, context: &Context) -> eval::Result {
        let condition = try!(
            self.cond.eval(&context).and_then(api::conv::bool)
        ).unwrap_bool();
        if condition {
            self.then.eval(&context)
        } else {
            self.else_.eval(&context)
        }
    }
}
