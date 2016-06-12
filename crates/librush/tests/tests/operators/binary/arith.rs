//! Tests for arithmetic operators.


mod plus {
    use util::*;

    #[test]
    fn constant_integers() {
        assert_eq!("0", eval("0 + 0"));
        assert_eq!("2", eval("0 + 2"));
        assert_eq!("4", eval("2 + 2"));
        assert_eq!("42", eval("-2 + 44"));
    }

    #[test]
    fn constant_floats() {
        assert_eq!("0.0", eval("0.0 + 0.0"));
        assert_eq!("2.0", eval("0 + 2.0"));
        assert_eq!("4.0", eval("2.0 + 2.0"));
        assert_eq!("42.0", eval("-2.5 + 44.5"));
    }

    #[test]
    fn constant_strings() {
        assert_eq!("foo", eval("\"\" + foo"));
        assert_eq!("foobar", eval("foo + bar"));
        assert_eq!("barbaz", eval("bar + \"baz\""));
    }

    #[test]
    fn input_integers() {
        assert_noop_apply("_ + 0", "42");
        assert_noop_apply("0 + _", "42");
        assert_eq!("42", apply("_ + 40", "2"));
        assert_eq!("42", apply("40 + _", "2"));
        assert_eq!("6", apply("_ + _", "3"));
        assert_eq!("12", apply("_ + _ + _", "4"));
    }
    // TODO(xion): input_floats
    // TODO(xion): input_strings
}

mod minus {
    use util::*;

    #[test]
    fn constant_integers() {
        assert_eq!("0", eval("0 - 0"));
        assert_eq!("2", eval("2 - 0"));
        assert_eq!("3", eval("5 - 2"));
        assert_eq!("-4", eval("1 - 5"));
        assert_eq!("-2", eval("-1 - 1"));
        assert_eq!("1", eval("-3 - -4"));
    }

    #[test]
    fn constant_floats() {
        assert_eq!("0.0", eval("0.0 - 0.0"));
        assert_eq!("2.0", eval("2.0 - 0.0"));
        assert_eq!("3.0", eval("5.0 - 2.0"));
        assert_eq!("-4.0", eval("1.0 - 5.0"));
        assert_eq!("-2.0", eval("-1.0 - 1.0"));
        assert_eq!("1.0", eval("-3.0 - -4.0"));
    }

    #[test]
    fn input_integers() {
        assert_noop_apply("_ - 0", "42");
        assert_eq!("-42", apply("0 - _", "42"));
        assert_eq!("40", apply("42 - _", "2"));
        assert_eq!("-2", apply("40 - _", "42"));
        assert_eq!("0", apply("_ - _", "42"));
        assert_eq!("-42", apply("_ - _ - _", "42"));
        assert_noop_apply("_ - (_ - _)", "42");
    }
    // TODO(xion): input_floats
}

mod times {
    use util::*;

    #[test]
    fn constant_integers() {
        assert_eq!("0", eval("0 * 0"));
        assert_eq!("0", eval("2 * 0"));
        assert_eq!("3", eval("3 * 1"));
        assert_eq!("-4", eval("4 * -1"));
        assert_eq!("2", eval("-2 * -1"));
    }

    #[test]
    fn constant_floats() {
        assert_eq!("0.0", eval("0.0 * 0.0"));
        assert_eq!("0.0", eval("2.0 * 0.0"));
        assert_eq!("3.0", eval("3.0 * 1.0"));
        assert_eq!("-4.0", eval("4.0 * -1.0"));
        assert_eq!("2.0", eval("-2.0 * -1.0"));
    }
    // TODO: string * integer
}

// TODO(xion): tests for division
