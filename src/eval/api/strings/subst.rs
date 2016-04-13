//! String substitution functions.

use regex::{self, Captures, Regex};

use eval::{self, Context, Error, Value};
use eval::api::conv::str_;
use eval::model::function::{Args, Invoke};
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
            &Value::String(ref r),
            &Value::String(ref h)) = (&needle, &replacement, &haystack) {
        return Ok(Value::String(n.replace_all(h, &r as &str)));
    }
    if let (&Value::Regex(ref n),
            &Value::Function(ref f),
            &Value::String(ref h)) = (&needle, &replacement, &haystack) {
        // the function should accept the value of each capture group;
        // note that the 0th one is the whole matched string
        if !f.arity().accepts(n.captures_len()) {
            return Err(Error::new(&format!(
                "replacement function in sub() must accept all \
                {} capture(s) as arguments, not just {}",
                n.captures_len(), f.arity()
            )));
        }

        // perform the replacement;
        // since the function that user passed may misbehave and e.g. return
        // a non-String Value, we need to account for potential errors
        let mut error: Option<Error> = None;
        let result = n.replace_all(h, |caps: &Captures| {
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
        });
        return match error {
            Some(e) => Err(e),
            _ => Ok(Value::String(result)),
        };
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

    // TODO(xion): the part below is repeated between sub() and sub1(),
    // and will probably also be almost identical for rsub1(); extract a helper

    // replacing regex matches with string or function
    if let (&Value::Regex(ref n),
            &Value::String(ref r),
            &Value::String(ref h)) = (&needle, &replacement, &haystack) {
        return Ok(Value::String(n.replace(h, &r as &str)));
    }
    if let (&Value::Regex(ref n),
            &Value::Function(ref f),
            &Value::String(ref h)) = (&needle, &replacement, &haystack) {
        // the function should accept the value of each capture group;
        // note that the 0th one is the whole matched string
        if !f.arity().accepts(n.captures_len()) {
            return Err(Error::new(&format!(
                "replacement function in sub() must accept all \
                {} capture(s) as arguments, not just {}",
                n.captures_len(), f.arity()
            )));
        }

        // perform the replacement;
        // since the function that user passed may misbehave and e.g. return
        // a non-String Value, we need to account for potential errors
        let mut error: Option<Error> = None;
        let result = n.replace(h, |caps: &Captures| {
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
        });
        return match error {
            Some(e) => Err(e),
            _ => Ok(Value::String(result)),
        };
    }

    Err(Error::new(&format!(
        "sub1() expects three strings; or regex, string/function and string; \
        got: {}, {}, {}",
        needle.typename(), replacement.typename(), haystack.typename()
    )))
}
