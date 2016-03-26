//! Conversion functions.

use csv;
use regex;

use eval::{self, Error, Value};
use eval::value::{BooleanRepr, IntegerRepr, FloatRepr, RegexRepr};


// Basic data types conversions

/// Convert a value to a boolean, based on its "truthy" value.
pub fn bool(value: Value) -> eval::Result {
    match value {
        Value::Boolean(_) => Ok(value),
        Value::Integer(i) => Ok(Value::Boolean(i != 0)),
        Value::Float(f) => Ok(Value::Boolean(f != 0.0)),
        Value::String(ref s) => s.parse::<BooleanRepr>()
            .map_err(|_| Error::new(&format!("invalid bool value: {}", s)))
            .map(Value::Boolean),
        Value::Array(ref a) => Ok(Value::Boolean(a.len() > 0)),
        Value::Object(ref o) => Ok(Value::Boolean(o.len() > 0)),
        _ => Err(Error::new(
            &format!("cannot convert {} to bool", value.typename())
        )),
    }
}

/// Convert a value to an integer.
pub fn int(value: Value) -> eval::Result {
    match value {
        Value::Boolean(b) => Ok(Value::Integer(if b { 1 } else { 0 })),
        Value::Integer(_) => Ok(value),
        Value::Float(f) => Ok(Value::Integer(f as IntegerRepr)),
        Value::String(ref s) => s.parse::<IntegerRepr>()
            .map_err(|_| Error::new(&format!("invalid integer value: {}", s)))
            .map(Value::Integer),
        _ => Err(Error::new(
            &format!("cannot convert {} to int", value.typename())
        )),
    }
}

/// Convert a value to a float.
pub fn float(value: Value) -> eval::Result {
    match value {
        Value::Boolean(b) => Ok(Value::Float(if b { 1.0 } else { 0.0 })),
        Value::Integer(i) => Ok(Value::Float(i as FloatRepr)),
        Value::Float(_) => Ok(value),
        Value::String(ref s) => s.parse::<FloatRepr>()
            .map_err(|_| Error::new(&format!("invalid float value: {}", s)))
            .map(Value::Float),
        _ => Err(Error::new(
            &format!("cannot convert {} to float", value.typename())
        )),
    }
}

/// Convert a value to string.
pub fn str_(value: Value) -> eval::Result {
    match value {
        Value::Boolean(b) => Ok(Value::String((
            if b { "true" } else { "false" }
        ).to_owned())),
        Value::Integer(i) => Ok(Value::String(i.to_string())),
        Value::Float(f) => Ok(Value::String(f.to_string())),
        Value::String(_) => Ok(value),
        Value::Regex(ref r) => Ok(Value::String(r.as_str().to_owned())),
        _ => Err(Error::new(
            &format!("cannot convert {} to string", value.typename())
        )),
    }
}

/// Convert a value to a regular expression.
/// If not a string, the value will be stringified first.
pub fn regex(value: Value) -> eval::Result {
    if value.is_regex() {
        return Ok(value);
    }

    // handle strings separately because we don't want to regex-escape them
    if value.is_string() {
        let value = value.unwrap_string();
        return RegexRepr::new(&value)
            .map(Value::Regex)
            .map_err(|e| Error::new(&format!(
                "invalid regular expression: {}", e)));
    }

    let value_type = value.typename();
    str_(value)
        .map(|v| regex::quote(&v.unwrap_string()))
        .and_then(|s| RegexRepr::new(&s).map_err(|e| {
            Error::new(&format!("cannot compile regular expression: {}", e))
        }))
        .map(Value::Regex)
        .map_err(|e| Error::new(&format!(
            "cannot convert {} to regular expression: {}", value_type, e
        )))
}


// Serialization formats conversions

/// Converts a value to or from CSV:
/// * string input is converted from CSV into an array (of arrays) of strings
/// * array input is converted to CSV string
pub fn csv(value: Value) -> eval::Result {
    eval1!((value: &String) -> Array {{
        let mut reader = csv::Reader::from_string(value as &str)
            .flexible(true)  // allow rows to have variable number of fields
            .has_headers(false)
            .record_terminator(csv::RecordTerminator::CRLF);

        // if we have been given a single line of CSV without the terminating
        // newline, return it as a single row
        // TODO(xion): cross-platform line ending detection
        if value.find("\n").is_none() {
            let record = reader.records().next().unwrap();
            let row = record.unwrap();
            row.into_iter().map(Value::String).collect()
        } else {
            // otherwise, return the parsed CSV as array of array of strings
            let mut result: Vec<Value> = Vec::new();
            for row in reader.records() {
                result.push(Value::Array(
                    row.unwrap().into_iter().map(Value::String).collect()
                ));
            }
            result
        }
    }});

    Err(Error::new(
        &format!("csv() expects string or array, got {}", value.typename())
    ))
}

// TODO(xion): json()
