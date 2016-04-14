//! Base API functions.

use eval::{self, Context, Error, Function, Value};
use eval::model::{ArgCount, Invoke};
use eval::value::IntegerRepr;
use parse::ast::BinaryOpNode;
use super::conv::bool;


/// Compute the length of given value (an array or a string).
pub fn len(value: Value) -> eval::Result {
    eval1!((value: &String) -> Integer { value.len() as IntegerRepr });
    eval1!((value: &Array) -> Integer { value.len() as IntegerRepr });
    eval1!((value: &Object) -> Integer { value.len() as IntegerRepr });
    Err(Error::new(&format!(
        "len() requires string/array/object, got {}", value.typename()
    )))
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

        (elem, seq) => Err(
            Error::new(&format!(
                "invalid arguments to index() function: {} and {}",
                elem.typename(), seq.typename()))
        ),
    }
}


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


/// Map a function over an array.
/// Returns the array created by applying the function to each element.
pub fn map(func: Value, array: Value, context: &Context) -> eval::Result {
    let array_type = array.typename();

    eval2!((func: &Function, array: Array) -> Array {{
        try!(ensure_argcount(&func, 1, "map"));

        let mut result = Vec::new();
        for item in array.into_iter() {
            let context = Context::with_parent(&context);
            let mapped = try!(func.invoke(vec![item], &context));
            result.push(mapped);
        }
        result
    }});

    Err(Error::new(&format!(
        "map() requires a function and an array, got {} and {}",
        func.typename(), array_type
    )))
}

/// Filter an array through a predicate function.
///
/// Returns the array created by apply the function to each element
/// and preserving only those for it returned a truthy value.
pub fn filter(func: Value, array: Value, context: &Context) -> eval::Result {
    let array_type = array.typename();

    eval2!((func: &Function, array: Array) -> Array {{
        try!(ensure_argcount(&func, 1, "filter"));

        let mut result = Vec::new();
        for item in array.into_iter() {
            let context = Context::with_parent(context);
            let keep = try!(
                func.invoke(vec![item.clone()], &context).and_then(bool)
            ).unwrap_bool();
            if keep {
                result.push(item);
            }
        }
        result
    }});

    Err(Error::new(&format!(
        "filter() requires a function and an array, got {} and {}",
        func.typename(), array_type
    )))
}

/// Apply a binary function cumulatively to array elements.
/// Also known as the "fold" operation (left fold, to be precise).
pub fn reduce(func: Value, array: Value, start: Value, context: &Context) -> eval::Result {
    let func_type = func.typename();
    let array_type = array.typename();

    if let (Value::Function(func), Value::Array(array)) = (func, array) {
        try!(ensure_argcount(&func, 2, "reduce"));

        let mut result = start;
        for item in array.into_iter() {
            let context = Context::with_parent(context);
            result = try!(func.invoke(vec![result, item], &context));
        }
        return Ok(result);
    }

    Err(Error::new(&format!(
        "reduce() requires a function and an array, got {} and {}",
        func_type, array_type
    )))
}


// Utility functions

#[inline(always)]
fn ensure_argcount(func: &Function, argcount: ArgCount, api_call: &str) -> Result<(), Error> {
    let arity = func.arity();
    if !arity.accepts(argcount) {
        return Err(Error::new(&format!(
            "{}() requires a {}-argument function, got one with {} arguments",
            api_call, argcount, arity
        )));
    }
    Ok(())
}
