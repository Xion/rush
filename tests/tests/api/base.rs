//! Tests for the base API functions.
#![allow(non_snake_case)]

use std::collections::HashMap;

use util::*;


#[test]
fn len() {
    const STRING: &'static str = "Hello, world";
    const ARRAY: &'static [u32] = &[1, 1, 2, 3, 5, 8, 13, 21];
    let OBJECT: HashMap<String, u32> = hashmap_owned!{"foo" => 1, "bar" => 2};

    assert_eval_error("len(42)");
    assert_eval_error("len(3.14)");
    assert_eq!("3", eval("len(foo)"));
    assert_eq!(STRING.len().to_string(), apply("len(_)", STRING));
    assert_eq!(ARRAY.len().to_string(),
               eval(&format!("len({})", to_array_literal(ARRAY))));
    assert_eq!(OBJECT.len().to_string(),
               eval(&format!("len({})", to_object_literal(&OBJECT))));
    assert_eval_error("len(|x| x)");
}

// TODO(xion): tests for index()
// TODO(xion): tests for all() and any()
// TODO(xion): tests for map() and filter()
