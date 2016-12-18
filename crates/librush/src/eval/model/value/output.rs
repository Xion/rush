//! Producing output from Value.
//!
//! This module defines how a Value is serialized as an output of the expression.
//! See also the `json` module.

#![allow(useless_format)]

use std::fmt;

use conv::TryFrom;
use conv::errors::GeneralError;
use rustc_serialize::json::ToJson;

use super::Value;


impl TryFrom<Value> for String {
    type Err = GeneralError<&'static str>;

    #[inline]
    fn try_from(src: Value) -> Result<Self, Self::Err> {
        String::try_from(&src)
    }
}

impl<'a> TryFrom<&'a Value> for String {
    type Err = <String as TryFrom<Value>>::Err;

    /// Try to convert a Value to string that can be emitted
    /// as a final result of a computation.
    fn try_from(src: &'a Value) -> Result<Self, Self::Err> {
        match *src {
            Value::Empty => Err(GeneralError::Unrepresentable(
                "cannot serialize an empty value"
            )),
            Value::Symbol(ref t) => Ok(format!("{}", t)),
            Value::Boolean(ref b) => Ok(format!("{}", b)),
            Value::Integer(ref i) => Ok(format!("{}", i)),
            Value::Float(ref f) => {
                // always include decimal point and zero, even if the float
                // is actually an integer
                let mut res = f.to_string();
                if !res.contains('.') {
                    res.push_str(".0");
                }
                Ok(res)
            },
            Value::String(ref s) => Ok(s.clone()),
            Value::Regex(..) => Err(GeneralError::Unrepresentable(
                "cannot serialize a regex"
            )),
            Value::Array(ref a) => {
                // for final display, an array is assumed to contain lines of output
                Ok(format!("{}", a.iter()
                    .map(|v| format!("{}", v)).collect::<Vec<String>>()
                    .join("\n")))
            },
            Value::Object(..) => Ok(src.to_json().to_string()),
            Value::Function(..) => Err(GeneralError::Unrepresentable(
                "cannot serialize a function"
            )),
        }
    }
}


impl fmt::Display for Value {
    /// Format a Value for outputing it as a result of the computation.
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        String::try_from(self)
            .map(|s| write!(fmt, "{}", s))
            // TODO(xion): return an Err(fmt::Error) rather than panicking
            // when formatting constructs actually react to it constructively
            .expect(&format!("can't display a value of type `{}`", self.typename()))
    }
}
