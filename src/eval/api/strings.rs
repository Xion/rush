//! String API available to expressions.

use std::error::Error as _Error;  // just for description() method
use std::fmt::Display;

use eval::{self, Error, Value};
use eval::value::StringRepr;
use eval::util::fmt::format;
use super::conv::str_;


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
        string.split(delim).map(StringRepr::from).map(Value::String).collect()
    });
    Err(Error::new(&format!(
        "split() expects two strings, got: {}, {}",
        string.typename(), delim.typename()
    )))
}

/// Join an array of values into a single delimited string.
pub fn join(array: Value, delim: Value) -> eval::Result {
    let array_type = array.typename();
    let delim_type = delim.typename();

    if let (Value::Array(a), Value::String(d)) = (array, delim) {
        let elem_count = a.len();
        let strings: Vec<_> =  a.into_iter()
            .map(str_).filter(Result::is_ok)
            .map(Result::unwrap).map(Value::unwrap_string)
            .collect();
        let error_count = strings.len() - elem_count;
        if error_count == 0 {
            return Ok(Value::String(strings.join(&d)));
        } else {
            return Err(Error::new(&format!(
                "join() failed to stringify {} element(s) of the input array",
                error_count)));
        }
    }

    Err(Error::new(&format!(
        "join() expects an array and string, got: {}, {}",
        array_type, delim_type
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


/// Peforms string formatting a'la Python str.format().
pub fn format_(fmt: Value, arg: Value) -> eval:: Result {
    if let Value::String(fmt) = fmt {
        let mut args: Vec<&Display> = Vec::new();

        match &arg {
            &Value::Empty |
            &Value::Symbol(..) |
            &Value::Function(..) => return Err(Error::new(&format!(
                "invalid argument for string formatting: {}", arg.typename()
            ))),
            &Value::Object(..) => {
                // TODO(xion): Object should be possible but the formatting code
                // doesn't support named placeholders yet :(
                return Err(Error::new(
                    "objects are not supported as string formatting arguments"
                ));
            },
            &Value::Array(ref a) => {
                args = a.iter().map(|v| v as &Display).collect();
            },
            _ => args.push(&arg),
        }

        return format(&fmt, &args)
            .map(Value::String)
            .map_err(|e| Error::new(&format!(
                "string formatting error: {}", e.description()
            )));
    }

    Err(Error::new(&format!(
        "format() expects a format string, got {}", fmt.typename()
    )))
}
