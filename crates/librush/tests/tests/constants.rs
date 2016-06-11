//! Tests for constant expressions.

use util::*;


#[test]
fn constant_boolean_true() {
    assert_noop_eval("true");
}

#[test]
fn constant_boolean_false() {
    assert_noop_eval("false");
}

mod numbers {
    use util::*;

    #[test]
    fn integer() {
        assert_noop_eval("42");
    }

    #[test]
    fn integer_negative() {
        // Note that this may actually be interpreted as unary minus expression,
        // but the user wouldn't care about that so we consider it constant.
        assert_noop_eval("-42");
    }

    #[test]
    fn float() {
        assert_noop_eval("42.42");
    }

    #[test]
    fn float_zero() {
        assert_noop_eval("0.0");
    }

    #[test]
    fn float_fraction() {
        assert_noop_eval("0.42");
    }

    #[test]
    fn float_scientific() {
        const EXPR: &'static str = "42.4e2";
        let expected = EXPR.parse::<f64>().unwrap().to_string() + ".0";
        assert_eq!(expected, eval(EXPR));
    }

    #[test]
    fn float_negative() {
        // Note that this may actually be interpreted as unary minus expression,
        // but the user wouldn't care about that so we consider it constant.
        assert_noop_eval("-42.42");
    }
}


mod string {
    use util::*;

    #[test]
    fn bare() {
        assert_noop_eval("foo");
    }

    #[test]
    fn quoted() {
        const STRING: &'static str = "foo";
        let expr = &format!("\"{}\"", STRING);
        assert_eq!(STRING, eval(expr));
        // TODO(xion): test escape sequences
    }
}

mod array {
    use util::*;

    #[test]
    fn empty() {
        const EXPR: &'static str = "[]";
        let expected = "";
        assert_eq!(expected, eval(EXPR));
    }

    #[test]
    fn one_element() {
        const ELEMENT: &'static str = "foo";
        let expr = format!("[{}]", ELEMENT);
        assert_eq!(ELEMENT, eval(&expr));
    }

    #[test]
    fn integers() {
        const ELEMENTS: &'static [i64] = &[13, 42, 100, 256];
        let expr = format!("[{}]", join(ELEMENTS, ","));
        let actual: Vec<_> = eval(&expr)
            .split('\n').map(|s| s.parse::<i64>().unwrap()).collect();
        assert_eq!(ELEMENTS, &actual[..]);
    }

    #[test]
    fn floats() {
        const ELEMENTS: &'static [f64] = &[-13.5, 0.00002, 42.007, 999999999.7];
        let expr = format!("[{}]", join(ELEMENTS, ","));
        let actual: Vec<_> = eval(&expr)
            .split('\n').map(|s| s.parse::<f64>().unwrap()).collect();
        assert_eq!(ELEMENTS, &actual[..]);
    }

    #[test]
    fn strings() {
        const ELEMENTS: &'static [&'static str] = &["foo", "bar", "baz"];
        let expr = format!("[{}]", join(ELEMENTS, ","));
        let actual: Vec<_> = eval(&expr).split('\n').map(String::from).collect();
        assert_eq!(ELEMENTS, &actual[..]);
    }

    #[test]
    fn quoted_strings() {
        const ELEMENTS: &'static [&'static str] = &["Alice", "has", "a", "cat"];
        let expr = format!("[{}]", ELEMENTS.iter()
            .map(|s| format!("\"{}\"", s)).collect::<Vec<_>>().join(","));
        let actual: Vec<_> = eval(&expr).split('\n').map(String::from).collect();
        assert_eq!(ELEMENTS, &actual[..]);
    }

    #[test]
    fn booleans() {
        const ELEMENTS: &'static [bool] = &[true, false, false, true, true];
        let expr = format!("[{}]", join(ELEMENTS, ","));
        let actual: Vec<_> = eval(&expr)
            .split('\n').map(|s| s.parse::<bool>().unwrap()).collect();
        assert_eq!(ELEMENTS, &actual[..]);
    }
}

mod object {
    use util::*;

    #[test]
    fn empty() {
        assert_noop_eval("{}");
    }

    #[test]
    fn one_attribute() {
        assert_noop_eval("{\"a\":2}");
        assert_eval_error("{2: 3}");  // because key has to be string
    }

    #[test]
    fn constant_object() {
        let elems = hashmap_owned!{"a" => "foo", "b" => "bar"};
        let actual = parse_json_stringmap(&eval(&elems.to_literal()));
        assert_eq!(elems, actual);
    }

    #[test]
    fn duplicate_key() {
        let key = "a";
        let first_value = "foo";
        let second_value = "bar";
        let elems = hashmap_owned!{key => first_value, key => second_value};

        let actual = parse_json_stringmap(&eval(&elems.to_literal()));
        assert!(actual.contains_key(key));
        assert_eq!(second_value, actual.get(key).unwrap());
    }
}
