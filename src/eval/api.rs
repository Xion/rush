//! API that's available out-of-the-box to the expressions.
//! It is essentially the standard library of the language.

use rand as _rand;

use eval::{self, Error};
use super::model::Value;


/// Compute the length of given value (an array or a string).
pub fn len(value: Value) -> eval::Result {
    eval1!((value: &String) -> Integer { value.len() as i64 });
    eval1!((value: &Array) -> Integer { value.len() as i64 });
    Err(Error::new(&format!(
        "len() requires string or array, got {}", value.typename()
    )))
}

/// Compute the absolute value of a number.
pub fn abs(value: Value) -> eval::Result {
    eval1!(value : Integer { value.abs() });
    eval1!(value : Float { value.abs() });
    Err(Error::new(&format!(
        "abs() requires a number, got {}", value.typename()
    )))
}

/// Generate a random floating point number from the 0..1 range.
pub fn rand() -> eval::Result {
    Ok(Value::Float(_rand::random::<f64>()))
}


// Conversions

/// Convert a value to string.
pub fn str(value: Value) -> eval::Result {
    value.to_string_value().ok_or_else(|| Error::new(
        &format!("cannot convert {} to string", value.typename())
    ))
}

/// Convert a value to an integer.
pub fn int(value: Value) -> eval::Result {
     value.to_int_value().ok_or_else(|| Error::new(
        &format!("cannot convert {} to int", value.typename())
    ))
}

/// Convert a value to a float.
pub fn float(value: Value) -> eval::Result {
    value.to_float_value().ok_or_else(|| Error::new(
        &format!("cannot convert {} to float", value.typename())
    ))
}

/// Convert a value to a boolean, based on its "truthy" value.
pub fn bool(value: Value) -> eval::Result {
    value.to_bool_value().ok_or_else(|| Error::new(
        &format!("cannot convert {} to bool", value.typename())
    ))
}


// Strings

/// Reverse the character in a string.
pub fn rev(string: Value) -> eval::Result {
    // TODO(xion): since this reverses chars not graphemes,
    // it mangles some non-Latin strings;
    // fix with unicode-segmentation crate
    eval1!(string : &String { string.chars().rev().collect() });
    Err(Error::new(&format!(
        "rev() requires a string, got {}", string.typename()
    )))
}

/// Split a string by given string delimiter.
/// Returns an array of strings.
pub fn split(string: Value, delim: Value) -> eval::Result {
    eval2!((string: &String, delim: &String) -> Array {
        string.split(delim).map(str::to_owned).map(Value::String).collect()
    });
    Err(Error::new(&format!(
        "split() expects two strings, got: {}, {}",
        string.typename(), delim.typename()
    )))
}

/// Join an array of values into a single delimited string.
pub fn join(array: Value, delim: Value) -> eval::Result {
    if let (&Value::Array(ref a),
            &Value::String(ref d)) = (&array, &delim) {
        let strings: Vec<_> =  a.iter()
            .map(Value::to_string_value).filter(Option::is_some)
            .map(Option::unwrap).map(Value::unwrap_string)
            .collect();
        if strings.len() == a.len() {
            return Ok(Value::String(strings.join(&d)));
        }
        // TODO(xion): error if not every element stringifies correctly
    }
    Err(Error::new(&format!(
        "join() expects an array and string, got: {}, {}",
        array.typename(), delim.typename()
    )))
}

/// Substitute a given string ("needle") with another ("replacement")
/// within given text ("haystack").
/// Returns the text after substitution has been made.
// TODO(xion): allow this function to accept just two arguments,
// with the third one being an implicit reference to the default var
// (requires allowing functions to access the Context)
pub fn sub(needle: Value, replacement: Value, haystack: Value) -> eval::Result {
    if let (&Value::String(ref n),
            &Value::String(ref r),
            &Value::String(ref h)) = (&needle, &replacement, &haystack) {
        return Ok(Value::String(h.replace(n, r)));
    }
    Err(Error::new(&format!(
        "sub() expects three strings, got: {}, {}, {}",
        needle.typename(), replacement.typename(), haystack.typename()
    )))
}
