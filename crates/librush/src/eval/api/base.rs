//! Base API functions.
//!
//! This is mostly a grab bag of functions that don't really fit anywhere else,
//! or are singularly "fundamental" to the expression language.

use std::collections::HashSet;
use std::cmp::Ordering;
use std::mem;

use conv::misc::InvalidSentinel;
use unicode_segmentation::UnicodeSegmentation;

use eval::{self, Context, Error, Value};
use eval::model::Invoke;
use eval::util::cmp::TryOrd;
use eval::value::{ArrayRepr, IntegerRepr, ObjectRepr, StringRepr};
use super::conv::{int, str_};


/// Compute the length of given value (an array or a string).
pub fn len(value: Value) -> eval::Result {
    eval1!((value: &String) -> Integer { value.chars().count() as IntegerRepr });
    eval1!((value: &Array) -> Integer { value.len() as IntegerRepr });
    eval1!((value: &Object) -> Integer { value.len() as IntegerRepr });
    mismatch!("len"; ("string") | ("array") | ("object") => (value))
}

/// Reverse the value.
///
/// For strings. this reverses the order of characters (Unicode grapheme clusters, to be precise).
/// For arrays, the result is an array with reversed elements.
///
/// For objects, the result is an inverted object, i.e. one where values from input
/// map back to keys. If there is more than one value for any given key,
/// it is undefined which one will be present in the result.
pub fn rev(value: Value) -> eval::Result {
    eval1!(value : &String {
        value.graphemes(/* extended grapheme clusters */ true)
            .rev()
            .collect::<Vec<_>>().join("")
    });

    if value.is_array() {
        return Ok(Value::Array(
            value.unwrap_array().into_iter().rev().collect()
        ));
    }

    // when reversing an object, the values need to be string-convertible
    if value.is_object() {
        let mut result = ObjectRepr::new();
        for (k, v) in value.unwrap_object() {
            let new_k = try!(str_(v)).unwrap_string();
            let new_v = Value::String(k);
            result.insert(new_k, new_v);
        }
        return Ok(Value::Object(result));
    }

    mismatch!("rev"; ("string") | ("array") | ("object") => (value))
}


/// Return an array of object's keys.
/// If a string or array is passed, an array of indices is returned.
pub fn keys(value: Value) -> eval::Result {
    // object is the main case; return the array of string keys
    if value.is_object() {
        return Ok(Value::Array(
            value.unwrap_object().into_iter()
                .map(|(k, _)| k).map(Value::String)
                .collect()
        ));
    }

    // for other indexable values, return an array of indices
    if value.is_array() {
        return Ok(Value::Array(
            (0..value.unwrap_array().len())
                .map(|i| i as IntegerRepr).map(Value::Integer)
                .collect()
        ));
    }
    if value.is_string() {
        return Ok(Value::Array(
            (0..value.unwrap_string().len())
                .map(|i| i as IntegerRepr).map(Value::Integer)
                .collect()
        ));
    }

    mismatch!("keys"; ("object") | ("array") | ("string") => (value))
}

/// Return an array of object's values.
pub fn values(value: Value) -> eval::Result {
    if value.is_object() {
        return Ok(Value::Array(
            value.unwrap_object().into_iter().map(|(_, v)| v).collect()
        ));
    }
    mismatch!("values"; ("object") => (value))
}


/// Pick only the values from given collection that match the keys.
/// Keys should be given as an array.
/// Source collection can be:
/// * an array (with element indices as keys)
/// * an object
/// * a string (with character indices as keys)
pub fn pick(keys: Value, from: Value) -> eval::Result {
    argcheck!("pick"; ("array", "string") |
                      ("array", "array") |
                      ("array", "object") => (keys, from));

    let keys = keys.unwrap_array();
    match from {
        Value::String(s) => {
            let mut result = StringRepr::with_capacity(keys.len());
            if !keys.is_empty() {
                let chars: Vec<char> = s.chars().collect();
                for idx in keys {
                    let idx = try!(int(idx)).unwrap_int();
                    if !(0 <= idx && idx < s.len() as IntegerRepr) {
                        return Err(Error::new(&format!(
                            "string index out of range: {} > {}", idx, s.len()
                        )));
                    }
                    result.push(chars[idx as usize]);
                }
            }
            Ok(Value::String(result))
        },
        Value::Array(mut a) => {
            let mut result = ArrayRepr::with_capacity(keys.len());
            for idx in keys {
                let idx = try!(int(idx)).unwrap_int();
                if !(0 <= idx && idx < a.len() as IntegerRepr) {
                    return Err(Error::new(&format!(
                        "string index out of range: {} > {}", idx, a.len()
                    )));
                }
                // Thanks to this mem::replace(), we can pick the elements in linear time
                // with respect to the number to keys.
                let elem = mem::replace(&mut a[idx as usize], Value::invalid_sentinel());
                result.push(elem);
            }
            Ok(Value::Array(result))
        },
        Value::Object(mut o) => {
            let mut result = ObjectRepr::with_capacity(keys.len());
            for key in keys {
                let key = try!(str_(key)).unwrap_string();
                let value = match o.remove(&key) {
                    Some(v) => v,
                    None => return Err(Error::new(
                        &if result.contains_key(&key) {
                            format!("duplicate object key: {}", key)
                        } else {
                            format!("key doesn't exist in source object: {}", key)
                        }
                    )),
                };
                result.insert(key, value);
            }
            Ok(Value::Object(result))
        },
        _ => unreachable!(),
    }
}

/// Opposite of pick(). Preserves only the values from given collections
/// whose keys/indices *don't* match the keys/indices from given array.
/// Source collection can be:
/// * an array (with element indices as keys)
/// * an object
/// * a string (with character indices as keys)
pub fn omit(keys: Value, from: Value) -> eval::Result {
    argcheck!("omit"; ("array", "string") |
                      ("array", "array") |
                      ("array", "object") => (keys, from));

    if keys.as_array().is_empty() {
        return Ok(from);
    }

    // deal with objects first, as their keys are strings and not numeric indices
    if from.is_object() {
        let keys = keys.unwrap_array();
        let from = from.unwrap_object();

        // form the set of keys to omit, making they are all string(ish)
        let mut keyset = HashSet::with_capacity(keys.len());
        for key in keys {
            let key = try!(str_(key)).unwrap_string();
            keyset.insert(key);
        }

        // build the resulting object by excluding those keys
        // TODO: if the number of keys to exclude is small, we'd be better off
        // just removing them from original object
        let mut result = ObjectRepr::with_capacity(from.len() - keyset.len());
        for (key, value) in from {
            if !keyset.contains(&key) {
                result.insert(key, value);
            }
        }
        return Ok(Value::Object(result));
    }

    // for arrays and strings, sort the keys array and try to convert them to numeric indices
    let keys = try!(sort(keys)).unwrap_array();
    let keys = {
        let mut indices = Vec::with_capacity(keys.len());
        for key in keys {
            let index = try!(int(key)).unwrap_int();
            indices.push(index as usize);
        }
        indices
    };

    match from {
        Value::String(s) => {
            let mut result = StringRepr::with_capacity(s.len() - keys.len());
            {
                // Simultaneously go over the string and the array of keys (indices, really).
                // Since the latter is sorted, we can just move through it linearly
                // and skip the string char whose index matches the current key.
                let mut k = 0;  // index in `keys`
                for (i, ch) in s.chars().enumerate() {
                    if k < keys.len() && i == keys[k] {
                        k += 1;
                        continue;
                    }
                    result.push(ch);
                }
            }
            Ok(Value::String(result))
        },
        Value::Array(a) => {
            let mut result = ArrayRepr::with_capacity(a.len() - keys.len());
            {
                // (See the algorithm description in Value::String branch above).
                let mut k = 0;
                for (i, elem) in a.into_iter().enumerate() {
                    if k < keys.len() && i == keys[k] {
                        k += 1;
                        continue;
                    }
                    result.push(elem);
                }
            }
            Ok(Value::Array(result))
        },
        _ => unreachable!(),
    }
}


/// Find an index of given element inside a sequence.
/// Returns an empty value if the element couldn't be found.
pub fn index(elem: Value, seq: Value) -> eval::Result {
    match (elem, seq) {
        // searching through a string
        (Value::String(needle), Value::String(haystack)) => Ok(
            haystack.find(&needle)
                .map(|i| Value::Integer(i as IntegerRepr))
                .unwrap_or(Value::Empty)
        ),
        (Value::Regex(regex), Value::String(haystack)) => Ok(
            regex.find(&haystack)
                .map(|(i, _)| Value::Integer(i as IntegerRepr))
                .unwrap_or(Value::Empty)
        ),

        // searching through an array
        (elem, Value::Array(array)) => Ok(
            array.iter().position(|item| *item == elem)
                .map(|i| Value::Integer(i as IntegerRepr))
                .unwrap_or(Value::Empty)
        ),

        (elem, seq) => mismatch!("index"; ("string", "string") |
                                          ("regex", "string") |
                                          ("any value", "array") => (elem, seq)),
    }
}


/// Sort the array using a default comparison method.
///
/// The only kinds of sortable values are numbers (integers & floats (sans NaN))
/// and strings (alphabetically). They do not compare to each other, though.
///
/// Returns the array after sorting.
pub fn sort(array: Value) -> eval::Result {
    if let Value::Array(mut array) = array {
        let mut error: Option<Error> = None;
        array.sort_by(|a, b| a.try_cmp(b).unwrap_or_else(|e| {
            error = Some(e);
            UNUSED_ORDERING
        }));
        return match error {
            Some(e) => Err(e),
            _ => Ok(Value::Array(array)),
        };
    }
    Err(Error::new(&format!(
        "sort() expects an array, got {}", array.typename()
    )))
}

/// Sort the array using a comparator.
///
/// The comparator should be a function that takes two values and returns:
/// * a negative number - if the first value is lower than the second one
/// * zero - if the both values are equal
/// * a positive number - if the first value is greater than the second one
///
/// Returns the array after sorting.
pub fn sort_by(array: Value, cmp: Value, context: &Context) -> eval::Result {
    let array_type = array.typename();
    let cmp_type = cmp.typename();

    if let (Value::Array(mut array), Value::Function(cmp)) = (array, cmp) {
        let zero = Value::Integer(0);
        let mut error: Option<Error> = None;
        array.sort_by(|a, b| match cmp.invoke2(a.clone(), b.clone(), context) {
            Ok(ref x) if *x < zero => Ordering::Less,
            Ok(ref x) if *x == zero => Ordering::Equal,
            Ok(ref x) if *x > zero => Ordering::Greater,
            Ok(ref x) => {
                error = Some(Error::new(&format!(
                    "comparator must return a number, got {}", x.typename()
                )));
                UNUSED_ORDERING
            },
            Err(e) => { error = Some(e); UNUSED_ORDERING },
        });
        return match error {
            Some(e) => Err(e),
            _ => Ok(Value::Array(array)),
        };
    }

    Err(Error::new(&format!(
        "sortby() expects an array and a function, got {} and {}",
        array_type, cmp_type
    )))
}

/// Dummy unused Ordering value that's used when a sorting predicate
/// fails to evaluate without an error, or returns an invalid value.
const UNUSED_ORDERING: Ordering = Ordering::Less;
