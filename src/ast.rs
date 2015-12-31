//! Data structures representing the abstract syntax tree (AST)
//! of parsed expressions.

use std::str::FromStr;

use eval::{self, Eval, EvalResult, Context, Value};


pub struct ValueNode {
    pub value: Value,
}

impl FromStr for ValueNode {
    type Err = <Value as FromStr>::Err;

    fn from_str(s: &str) -> Result<ValueNode, Self::Err> {
        s.parse::<Value>().map(|v| ValueNode{value: v})
    }
}

impl Eval for ValueNode {
    fn eval(&self, context: &Context) -> EvalResult {
        Ok(self.resolve(&context))
    }
}

impl ValueNode {
    /// Resolve a possible variable reference against given context.
    ///
    /// Returns the variable's Value (which may be just variable name as string),
    /// or a copy of the original Value if it wasn't a reference.
    fn resolve(&self, context: &Context) -> Value {
        match self.value {
            Value::Reference(ref t) => context.get_var(t)
                .map(|v| v.clone())
                .unwrap_or_else(|| Value::String(t.clone())),
            _ => self.value.clone(),
        }
    }
}


pub struct BinaryOpNode {
    pub first: Box<Eval>,
    pub rest: Vec<(String, Box<Eval>)>,
}

impl Eval for BinaryOpNode {
    fn eval(&self, context: &Context) -> EvalResult {
        let mut result = try!(self.first.eval(&context));
        for &(ref op, ref arg) in &self.rest {
            let arg = try!(arg.eval(&context));
            match &op[..] {
                "+" => result = try!(BinaryOpNode::eval_plus(&result, &arg)),
                "-" => result = try!(BinaryOpNode::eval_minus(&result, &arg)),
                // TODO(xion): other operators
                _ => { return Err(
                    eval::Error::new(&format!("unknown operator: {}", op))
                ); }
            }
        }
        Ok(result)
    }
}

impl BinaryOpNode {
    /// Evaluate the "+" operator for two values.
    fn eval_plus(left: &Value, right: &Value) -> EvalResult {
        if let &Value::String(ref left) = left {
            if let &Value::String(ref right) = right {
                return Ok(Value::String(left.clone() + &*right));
            }
        }
        if let Value::Integer(left) = *left {
            if let Value::Integer(right) = *right {
                return Ok(Value::Integer(left + right));
            }
        }
        if let Value::Float(left) = *left {
            if let Value::Float(right) = *right {
                return Ok(Value::Float(left + right));
            }
        }
        Err(eval::Error::new("invalid types for (+) operator"))
    }

    /// Evaluate the "-" operator for two values.
    fn eval_minus(left: &Value, right: &Value) -> EvalResult {
        if let Value::Integer(left) = *left {
            if let Value::Integer(right) = *right {
                return Ok(Value::Integer(left - right));
            }
        }
        if let Value::Float(left) = *left {
            if let Value::Float(right) = *right {
                return Ok(Value::Float(left - right));
            }
        }
        Err(eval::Error::new("invalid types for (-) operator"))
    }
}


pub struct FunctionCallNode {
    pub name: String,
    pub args: Vec<Box<Eval>>,
}

impl Eval for FunctionCallNode {
    fn eval(&self, context: &Context) -> Result<Value, eval::Error> {
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
