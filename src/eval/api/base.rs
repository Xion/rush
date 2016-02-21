//! Base API functions.

use eval::{self, Context, Error, Value};
use eval::model::Invoke;
use eval::value::IntegerRepr;


/// Compute the length of given value (an array or a string).
pub fn len(value: Value) -> eval::Result {
    eval1!((value: &String) -> Integer { value.len() as IntegerRepr });
    eval1!((value: &Array) -> Integer { value.len() as IntegerRepr });
    eval1!((value: &Object) -> Integer { value.len() as IntegerRepr });
    Err(Error::new(&format!(
        "len() requires string or array, got {}", value.typename()
    )))
}


/// Map a function over an array.
/// Returns the array created by applying the function to each element.
pub fn map(func: Value, array: Value, context: &Context) -> eval::Result {
    let array_type = array.typename();

    eval2!((func: &Function, array: Array) -> Array {{
        let mut items = array;
        let mut result = Vec::new();
        for item in items.drain(..) {
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
