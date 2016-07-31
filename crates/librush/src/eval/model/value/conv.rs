//! Module handling conversions of Value to & from Rust types.

use std::convert::From;
use std::str::FromStr;

use conv::errors::NoError;

use super::Value;
use super::types::*;


/// Macro to create a straighforward From<FooRepr> -> Value::Foo implementation.
macro_rules! value_from (
    ($input:ty => $output:ident) => {
        impl From<$input> for Value {
            #[inline]
            fn from(input: $input) -> Self {
                Value::$output(input)
            }
        }
    }
);

// Note how string input is deliberately omitted, for it is ambiguous.
// (It could result in either Value::String or Value::Symbol).
value_from!(BooleanRepr => Boolean);
value_from!(IntegerRepr => Integer);
value_from!(FloatRepr => Float);
value_from!(RegexRepr => Regex);
value_from!(ArrayRepr => Array);
value_from!(ObjectRepr => Object);
value_from!(FunctionRepr => Function);


// Convert from characters to 1-character string values.
impl From<char> for Value {
    fn from(input: char) -> Self {
        let mut string = String::new();
        string.push(input);
        Value::String(string)
    }
}

// Convert from bytes into integer values of those bytes.
impl From<u8> for Value {
    fn from(input: u8) -> Self {
        Value::Integer(input as IntegerRepr)
    }
}


// This is a somewhat special "default" and "intelligent" conversion that's
// by default applied to the expresion's input (the _ symbol).
impl FromStr for Value {
    type Err = NoError;

    /// Create a Value from string, reinterpreting input as number
    /// if we find out it's in numeric form.
    fn from_str(s: &str) -> Result<Value, Self::Err> {
        if let Ok(int) = s.parse::<IntegerRepr>() {
            return Ok(Value::Integer(int));
        }
        if let Ok(float) = s.parse::<FloatRepr>() {
            return Ok(Value::Float(float));
        }
        if let Ok(boolean) = s.parse::<BooleanRepr>() {
            return Ok(Value::Boolean(boolean));
        }
        Ok(Value::String(s.to_owned()))
    }
}
