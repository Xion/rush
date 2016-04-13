//! String substitution functions.

use regex::{self, Captures, Regex};

use eval::{self, Context, Error, Value};
use eval::api::conv::str_;
use eval::model::{Args, Invoke};
use eval::value::StringRepr;


/// Substitute a given string or regex ("needle") with something else ("replacement")
/// within given text ("haystack").
///
/// The replacement can be either another string, or -- in case of regex needle --
/// a function accepting the values of regex captures and returning replacement string.
///
/// Returns the text after the substitutions has been made.
pub fn sub(needle: Value, replacement: Value, haystack: Value, ctx: &Context) -> eval::Result {
    // replacing string with string
    if let (&Value::String(ref n),
            &Value::String(ref r),
            &Value::String(ref h)) = (&needle, &replacement, &haystack) {
        return Ok(Value::String(h.replace(n, r)));
    }

    // replacing regex matches with string or function
    if let (&Value::Regex(ref n),
            &Value::String(ref h)) = (&needle, &haystack) {
        return do_regex_sub(Sub::All, n, &replacement, h, ctx);
    }

    Err(Error::new(&format!(
        "sub() expects three strings; or regex, string/function and string; \
        got: {}, {}, {}",
        needle.typename(), replacement.typename(), haystack.typename()
    )))
}

/// Substitute the first occurrence of given string or regex ("needle")
/// with something else ("replacement") within given text.
///
/// The replacement can be either another string, or -- in case of regex needle --
/// a function accepting the values of regex captures and returning replacement string.
///
/// Returns the text after the substitution has been made.
pub fn sub1(needle: Value, replacement: Value, haystack: Value, ctx: &Context) -> eval::Result {
    // replacing string with string
    if let (&Value::String(ref n),
            &Value::String(ref r),
            &Value::String(ref h)) = (&needle, &replacement, &haystack) {
        // TODO(xion): a String::find-based approach will likely be faster
        let needle_re = Regex::new(&regex::quote(n)).unwrap();
        return Ok(Value::String(needle_re.replace(h, &r as &str)));
    }

    // replacing regex matches with string or function
    if let (&Value::Regex(ref n),
            &Value::String(ref h)) = (&needle, &haystack) {
        return do_regex_sub(Sub::First, n, &replacement, h, ctx);
    }

    Err(Error::new(&format!(
        "sub1() expects three strings; or regex, string/function and string; \
        got: {}, {}, {}",
        needle.typename(), replacement.typename(), haystack.typename()
    )))
}


// Utility functions

/// Enum definining the kind of substitution to perform.
enum Sub {
    /// Replace all occurrences.
    All,
    /// Replace only the first occurrence.
    First,
}

/// Perform a regex-based substitution.
/// Replacement can be either a string or a function taking capture group values.
fn do_regex_sub(how: Sub,
                needle: &Regex, replacement: &Value, haystack: &String,
                ctx: &Context) -> eval::Result {
    if let &Value::String(ref r) = replacement {
        let result = match how {
            Sub::All => needle.replace_all(haystack, r as &str),
            Sub::First => needle.replace(haystack, r as &str),
        };
        return Ok(Value::String(result));
    }

    if let &Value::Function(ref f) = replacement {
        // the function should accept the value of each capture group;
        // note that the 0th one is the whole matched string
        if !f.arity().accepts(needle.captures_len()) {
            return Err(Error::new(&format!(
                "replacement function in sub() must accept all \
                {} capture(s) as arguments, not just {}",
                needle.captures_len(), f.arity()
            )));
        }

        // perform the replacement;
        // since the function that user passed may misbehave and e.g. return
        // a non-String Value, we need to account for potential errors
        let mut error: Option<Error> = None;
        let result = {
            let replacement_func = |caps: &Captures| {
                let args: Args = caps.iter().map(|c| {
                    c.map(StringRepr::from).map(Value::String).unwrap_or(Value::Empty)
                }).collect();

                let result = f.invoke(args, &ctx)
                    .and_then(str_).map(Value::unwrap_string);
                match result {
                    Ok(s) => s,
                    Err(e) => {
                        error = Some(e);
                        "__INVALID__".to_owned() // won't be used anyway
                    }
                }
            };
            match how {
                Sub::All => needle.replace_all(haystack, replacement_func),
                Sub::First => needle.replace(haystack, replacement_func),
            }
        };
        return match error {
            Some(e) => Err(e),
            _ => Ok(Value::String(result)),
        };
    }

    Err(Error::new(&format!(
        "regex-based substitution requires string or function replacement, got {}",
        replacement.typename()
    )))
}
