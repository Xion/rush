//! Tests for the base API functions.
#![allow(non_snake_case)]

use std::collections::HashMap;

use util::*;


#[test]
fn len() {
    const STRING: &'static str = "Hello, world";
    const ARRAY: &'static [u32] = &[1, 1, 2, 3, 5, 8, 13, 21];
    let OBJECT: HashMap<String, u32> = hashmap_owned!{"foo" => 1, "bar" => 2};

    assert_eval_error("len(false)");
    assert_eval_error("len(42)");
    assert_eval_error("len(3.14)");
    assert_eq!("3", eval("len(foo)"));
    assert_eval_error("len(/foo/)");
    assert_eq!(STRING.len().to_string(), apply("len(_)", STRING));
    assert_eq!(ARRAY.len().to_string(),
               eval(&format!("len({})", ARRAY.to_literal())));
    assert_eq!(OBJECT.len().to_string(),
               eval(&format!("len({})", OBJECT.to_literal())));
    assert_eval_error("len(|x| x)");
}

mod rev {
    use util::*;

    #[test]
    fn strings() {
        assert_eq!("oof", apply("rev(_)", "foo"));
        assert_noop_apply("rev(_)", "racecar");
        assert_apply_error("rev(_)", "42");
        assert_apply_error("rev(_)", "13.42");
        assert_apply_error("rev(_)", "false");
    }

    #[test]
    fn arrays() {
        assert_eq!("", eval(&format!("rev({})", "[]")));
    }

    #[test]
    fn objects() {
        assert_eq!("{}", eval(&format!("rev({})", "{}")));
    }

    // TODO: more tests for arrays & objects
}

#[test]
fn keys() {
    const STRING: &'static str = "Hello, world";
    const ARRAY: &'static [u32] = &[1, 1, 2, 3, 5, 8, 13, 21];
    let OBJECT: HashMap<String, u32> = hashmap_owned!{"foo" => 1, "bar" => 2};

    assert_eval_error("keys(true)");
    assert_eval_error("keys(42)");
    assert_eval_error("keys(3.14)");
    assert_eval_error("keys(/foo/)");

    // for strings and arrays, the result is array of indices, in order
    assert_eq!(join(&(0..STRING.len()).collect::<Vec<_>>(), "\n"),
               eval(&format!("keys({})", STRING.to_literal())));
    assert_eq!(join(&(0..ARRAY.len()).collect::<Vec<_>>(), "\n"),
               eval(&format!("keys({})", ARRAY.to_literal())));

    // for objects, it's an array of keys (in any order)
    let object_keys = eval(&format!("keys({})", OBJECT.to_literal()));
    for key in OBJECT.keys() {
        assert!(object_keys.contains(key));
    }

    assert_eval_error("keys(|x| x)")
}
// TODO: tests for values()

mod index {
    use util::*;

    #[test]
    fn string() {
        const STRING: &'static str = "Hello, world";

        assert_eval_error(&format!("index(true, {})", STRING.to_literal()));
        assert_eval_error(&format!("index(42, {})", STRING.to_literal()));
        assert_eval_error(&format!("index(3.14, {})", STRING.to_literal()));

        assert_eq!("0", eval(&format!("index(Hell, {})", STRING.to_literal())));
        assert_eq!("6", eval(&format!("index(/ \\w+/, {})", STRING.to_literal())));

        assert_eval_error(&format!("index([], {})", STRING.to_literal()));
        assert_eval_error(&format!("index({{}}, {})", STRING.to_literal()));
        assert_eval_error(&format!("index(|x| x, {})", STRING.to_literal()));
    }

    #[test]
    fn array() {
        const ARRAY: &'static [u32] = &[1, 1, 2, 3, 5, 8, 13, 21];

        assert_eval_error(&format!("index(false, {})", ARRAY.to_literal()));

        assert_eq!("4", eval(&format!("index(5, {})", ARRAY.to_literal())));

        assert_eval_error(&format!("index(42, {})", ARRAY.to_literal()));
        assert_eval_error(&format!("index(2.71, {})", ARRAY.to_literal()));
        assert_eval_error(&format!("index(foo, {})", ARRAY.to_literal()));
        assert_eval_error(&format!("index([], {})", ARRAY.to_literal()));
        assert_eval_error(&format!("index({{}}, {})", ARRAY.to_literal()));
        assert_eval_error(&format!("index(|x| x, {})", ARRAY.to_literal()));
    }

    #[test]
    fn errors() {
        assert_eval_error("index(42, false)");
        assert_eval_error("index(42, 42)");
        assert_eval_error("index(42, 3.14)");
        assert_eval_error("index(42, foo)");
        assert_eval_error("index(42, /foo/)");
        assert_eval_error("index(42, [])");
        assert_eval_error("index(42, {})");
        assert_eval_error("index(42, [])");
        assert_eval_error("index(42, |x| x)");
    }
}

// TODO(xion): tests for sort() and sortby()
