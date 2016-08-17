//! Iteration-related API functions.

use eval::{self, Context, Error, Value};
use parse::ast::BinaryOpNode;
use super::conv::bool;


/// Returns true if all elements of an array are truthy (as per bool() functions).
/// Note that if the array is empty, it also returns true.
pub fn all(value: Value) -> eval::Result {
    let value_type = value.typename();

    eval1!((value: Array) -> Boolean {{
        let mut result = true;
        for elem in value.into_iter() {
            let truthy = try!(bool(elem)).unwrap_bool();
            if !truthy {
                result = false;
                break;
            }
        }
        result
    }});

    Err(Error::new(&format!("all() requires an array, got {}", value_type)))
}

/// Returns true if at least one element of the array is truthy
/// (as per the bool() function).
pub fn any(value: Value) -> eval::Result {
    let value_type = value.typename();

    eval1!((value: Array) -> Boolean {{
        let mut result = false;
        for elem in value.into_iter() {
            let truthy = try!(bool(elem)).unwrap_bool();
            if truthy {
                result = true;
                break;
            }
        }
        result
    }});

    Err(Error::new(&format!("any() requires an array, got {}", value_type)))
}


// TODO(xion): make min(), max() and sum() accept arbitrary number of scalars

/// Find a minimum value in the array. Returns nil for empty arrays.
pub fn min(value: Value, context: &Context) -> eval::Result {
    let value_type = value.typename();

    if let Value::Array(array) = value {
        if array.is_empty() {
            return Ok(Value::Empty);
        }

        let mut items = array.into_iter();
        let mut result = items.next().unwrap();
        for item in items {
            let is_less = try!(
                BinaryOpNode::eval_op("<", item.clone(), result.clone(), context)
            );
            if is_less.unwrap_bool() {
                result = item;
            }
        }
        return Ok(result);
    }

    Err(Error::new(&format!("min() requires an array, got {}", value_type)))
}

/// Find a maximum value in the array. Returns nil for empty arrays.
pub fn max(value: Value, context: &Context) -> eval::Result {
    let value_type = value.typename();

    if let Value::Array(array) = value {
        if array.is_empty() {
            return Ok(Value::Empty);
        }

        let mut items = array.into_iter();
        let mut result = items.next().unwrap();
        for item in items {
            let is_greater = try!(
                BinaryOpNode::eval_op(">", item.clone(), result.clone(), context)
            );
            if is_greater.unwrap_bool() {
                result = item;
            }
        }
        return Ok(result);
    }

    Err(Error::new(&format!("max() requires an array, got {}", value_type)))
}

/// Return a sum of all elements in an array.
pub fn sum(value: Value, context: &Context) -> eval::Result {
    let value_type = value.typename();

    if let Value::Array(array) = value {
        if array.is_empty() {
            return Ok(Value::Empty);
        }

        let mut items = array.into_iter();
        let mut result = items.next().unwrap();
        for item in items {
            result = try!(BinaryOpNode::eval_op("+", result, item, context));
        }
        return Ok(result);
    }

    Err(Error::new(&format!("sum() requires an array, got {}", value_type)))
}


/// Returns the array with all falsy values removed.
/// This is determined via the bool() conversion.
pub fn compact(array: Value) -> eval::Result {
    let array_type = array.typename();

    eval1!(array : Array {{
        let mut result = Vec::new();
        for item in array.into_iter() {
            let keep = try!(bool(item.clone())).unwrap_bool();
            if keep {
                result.push(item);
            }
        }
        result
    }});

    Err(Error::new(&format!(
        "compact() requires an array, got {}", array_type
    )))
}
