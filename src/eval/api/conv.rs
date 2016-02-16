//! Conversion functions.

use eval::{self, Error, Value};
use eval::value::{BooleanRepr, IntegerRepr, FloatRepr};


/// Convert a value to string.
pub fn str_(value: Value) -> eval::Result {
    match value {
        Value::String(_) => Ok(value),
        Value::Integer(i) => Ok(Value::String(i.to_string())),
        Value::Float(f) => Ok(Value::String(f.to_string())),
        Value::Boolean(b) => Ok(Value::String((
            if b { "true" } else { "false" }
        ).to_string())),
        _ => Err(Error::new(
            &format!("cannot convert {} to string", value.typename())
        )),
    }
}

/// Convert a value to an integer.
pub fn int(value: Value) -> eval::Result {
    match value {
        Value::String(ref s) => s.parse::<IntegerRepr>()
            .map_err(|_| Error::new(&format!("invalid integer value: {}", s)))
            .map(Value::Integer),
        Value::Integer(_) => Ok(value),
        Value::Float(f) => Ok(Value::Integer(f as IntegerRepr)),
        Value::Boolean(b) => Ok(Value::Integer(if b { 1 } else { 0 })),
        _ => Err(Error::new(
            &format!("cannot convert {} to int", value.typename())
        )),
    }
}

/// Convert a value to a float.
pub fn float(value: Value) -> eval::Result {
    match value {
        Value::String(ref s) => s.parse::<FloatRepr>()
            .map_err(|_| Error::new(&format!("invalid float value: {}", s)))
            .map(Value::Float),
        Value::Integer(i) => Ok(Value::Float(i as FloatRepr)),
        Value::Float(_) => Ok(value),
        Value::Boolean(b) => Ok(Value::Float(if b { 1.0 } else { 0.0 })),
        _ => Err(Error::new(
            &format!("cannot convert {} to float", value.typename())
        )),
    }
}

/// Convert a value to a boolean, based on its "truthy" value.
pub fn bool(value: Value) -> eval::Result {
    match value {
        Value::String(ref s) => s.parse::<BooleanRepr>()
            .map_err(|_| Error::new(&format!("invalid bool value: {}", s)))
            .map(Value::Boolean),
        Value::Integer(i) => Ok(Value::Boolean(i != 0)),
        Value::Float(f) => Ok(Value::Boolean(f != 0.0)),
        Value::Boolean(_) => Ok(value),
        Value::Array(ref a) => Ok(Value::Boolean(a.len() > 0)),
        _ => Err(Error::new(
            &format!("cannot convert {} to bool", value.typename())
        )),
    }
}
