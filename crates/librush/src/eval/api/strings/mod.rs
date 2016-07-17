//! String API available to expressions.

mod frag;
mod subst;

pub use self::frag::*;
pub use self::subst::*;


use std::char;
use std::error::Error as StdError;  // just for its description() method
use std::fmt::Display;
use std::str::from_utf8;

use eval::{self, Error, Value};
use eval::value::{IntegerRepr, StringRepr};
use eval::util::fmt::format;


/// Returns a one-character string with the character of given ordinal value.
pub fn chr(value: Value) -> eval::Result {
    eval1!((value: Integer) -> String where (value >= 0) {{
        let ord = value as u32;
        let ch = try!(char::from_u32(ord)
            .ok_or_else(|| Error::new(&format!(
                "invalid character ordinal: {}", ord
            ))));
        let mut result = String::with_capacity(1);
        result.push(ch);
        result
    }});
    Err(Error::new(&format!(
        "chr() expects a positive integer, got {}", value.typename()
    )))
}

/// Returns the ordinal value for a single character in a string.
pub fn ord(value: Value) -> eval::Result {
    eval1!((value: &String) -> Integer {
        match value.len() {
            1 => value.chars().next().unwrap() as IntegerRepr,
            len@_ => return Err(Error::new(&format!(
                "ord() requires string of length 1, got {}", len
            ))),
        }
    });
    Err(Error::new(&format!(
        "ord() expects a string, got {}", value.typename()
    )))
}


/// Converts a string into an array of characters.
/// Each character is represented as a string of length 1.
pub fn chars(value: Value) -> eval::Result {
    eval1!((value: &String) -> Array {
        value.chars()
            .map(|c| { let mut s = String::with_capacity(1); s.push(c); s })
            .map(Value::String)
            .collect()
    });
    mismatch!("chars"; ("string") => (value))
}


/// Peforms string formatting a'la Python str.format().
pub fn format_(fmt: Value, arg: Value) -> eval:: Result {
    if let Value::String(fmt) = fmt {
        let mut args: Vec<&Display> = Vec::new();

        match &arg {
            &Value::Boolean(..) |
            &Value::Integer(..) |
            &Value::Float(..) |
            &Value::String(..) => args.push(&arg),
            &Value::Array(ref a) => {
                args = a.iter().map(|v| v as &Display).collect();
            },
            &Value::Object(..) => {
                // TODO(xion): Object should be possible but the formatting code
                // doesn't support named placeholders yet :(
                return Err(Error::new(
                    "objects are not supported as string formatting arguments"
                ));
            },
            _ => return Err(Error::new(&format!(
                "invalid argument for string formatting: {}", arg.typename()
            ))),
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


/// Return part of a string ("haystack") before given one ("needle"),
/// or empty string if not found.
pub fn before(needle: Value, haystack: Value) -> eval::Result {
    eval2!((needle: &String, haystack: &String) -> String {
        match haystack.find(&needle as &str) {
            Some(index) => StringRepr::from(
                from_utf8(&haystack.as_bytes()[0..index]).unwrap()
            ),
            _ => String::new(),
        }
    });
    eval2!((needle: &Regex, haystack: &String) -> String {
        match needle.find(&haystack) {
            Some((index, _)) => StringRepr::from(
                from_utf8(&haystack.as_bytes()[0..index]).unwrap()
            ),
            _ => String::new(),
        }
    });

    Err(Error::new(&format!(
        "before() expects two strings, or regex and string, got {} and {}",
        needle.typename(), haystack.typename()
    )))
}

/// Return part of a string ("haystack") after given one ("needle"),
/// or empty string if not found.
pub fn after(needle: Value, haystack: Value) -> eval::Result {
    eval2!((needle: &String, haystack: &String) -> String {
        match haystack.find(&needle as &str) {
            Some(index) => StringRepr::from(
                from_utf8(&haystack.as_bytes()[index + needle.len()..]).unwrap()
            ),
            _ => String::new(),
        }
    });
    eval2!((needle: &Regex, haystack: &String) -> String {
        match needle.find(&haystack) {
            Some((_, index)) => StringRepr::from(
                from_utf8(&haystack.as_bytes()[index..]).unwrap()
            ),
            _ => String::new(),
        }
    });

    Err(Error::new(&format!(
        "after() expects two strings, or regex and string, got {} and {}",
        needle.typename(), haystack.typename()
    )))
}

/// Trim the string from whitespace characters at both ends.
pub fn trim(string: Value) -> eval::Result {
    eval1!(string : &String { string.trim().to_owned() });
    Err(Error::new(&format!(
        "trim() requires a string, got {}", string.typename()
    )))
}
