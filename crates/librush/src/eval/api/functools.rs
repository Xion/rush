//! API related to functions and "functional" programming.

use eval::{self, Context, Error, Function, Value};
use eval::model::{Args, ArgCount, Invoke};
use super::conv::bool;


/// Identity function.
pub fn identity(value: Value) -> eval::Result {
    Ok(value)
}

/// Create a function that invokes given one with arguments reversed.
pub fn flip(value: Value) -> eval::Result {
    if value.is_function() {
        let func = value.unwrap_function();
        let arity = func.arity();
        let flipped = move |mut args: Args, context: &Context| {
            args.reverse();
            func.invoke(args, context)
        };
        return Ok(Value::Function(Function::from_native_ctx(arity, flipped)));
    }
    mismatch!("flip"; ("function") => (value))
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
            let mapped = try!(func.invoke1(item, &context));
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
/// This is the opposite of reject().
///
/// Returns the array created by applying the function to each element
/// and preserving only those for it returned a truthy value.
pub fn filter(func: Value, array: Value, context: &Context) -> eval::Result {
    let array_type = array.typename();

    eval2!((func: &Function, array: Array) -> Array {{
        try!(ensure_argcount(&func, 1, "filter"));

        let mut result = Vec::new();
        for item in array.into_iter() {
            let context = Context::with_parent(context);
            let keep = try!(
                func.invoke1(item.clone(), &context).and_then(bool)
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

/// Reject array elements that do not satisfy a predicate.
/// This the opposite of filter().
///
/// Returns the array created by applying the function to each element
/// and preserving only those for it returned a falsy value.
pub fn reject(func: Value, array: Value, context: &Context) -> eval::Result {
    let array_type = array.typename();

    eval2!((func: &Function, array: Array) -> Array {{
        try!(ensure_argcount(&func, 1, "reject"));

        let mut result = Vec::new();
        for item in array.into_iter() {
            let context = Context::with_parent(context);
            let discard = try!(
                func.invoke1(item.clone(), &context).and_then(bool)
            ).unwrap_bool();
            if !discard {
                result.push(item);
            }
        }
        result
    }});

    Err(Error::new(&format!(
        "reject() requires a function and an array, got {} and {}",
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
            result = try!(func.invoke2(result, item, &context));
        }
        return Ok(result);
    }

    Err(Error::new(&format!(
        "reduce() requires a function and an array, got {} and {}",
        func_type, array_type
    )))
}


// Utility functions

#[inline]
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
