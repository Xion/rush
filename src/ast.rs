//! Data structures representing the abstract syntax tree (AST)
//! of parsed expressions.

use eval::{Eval, Context, Value};


pub struct ValueNode {
    pub value: Value,
}

impl Eval for ValueNode {
    fn eval(&self, context: &Context) -> Value {
        // treat the literal value as variable name if such variable exists;
        // otherwise, just return the value itself as string
        context.get_var(&self.value).unwrap_or(&self.value).clone()
    }
}


pub struct BinaryOpNode {
    pub op: String,  // TODO(xion): enum?
    pub left: Box<Eval>,
    pub right: Box<Eval>,
}

impl Eval for BinaryOpNode {
    fn eval(&self, context: &Context) -> Value {
        match &self.op[..] {
            "+" => {
                // TODO(xion): string concatenation vs. adding numbers
                self.left.eval(&context) + &self.right.eval(&context)
            }
            // TODO(xion): other operators
            _ => panic!("unknown operator: {}", self.op)
        }
    }
}
