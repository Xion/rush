//! Tests for unary operators.

mod plus {
    use util::*;

    #[test]
    fn integer() {
        assert_noop_apply("+_", "42");
        assert_noop_apply("++_", "42");
        assert_noop_apply("+++_", "42");
    }

    #[test]
    fn float() {
        assert_noop_apply("+_", "42.42");
        assert_noop_apply("++_", "42.42");
        assert_noop_apply("+++_", "42.42");
    }

    #[test]
    fn string() {
        assert_apply_error("+_", "foo");
    }

    #[test]
    fn boolean() {
        assert_apply_error("+_", "true");
        assert_apply_error("+_", "false");
    }
}

mod minus {
    use util::*;

    #[test]
    fn integer() {
        const INPUT: &'static str = "42";
        let negated = format!("-{}", INPUT);
        assert_eq!(negated, apply("-_", INPUT));
        assert_eq!(INPUT, apply("--_", INPUT));
        assert_eq!(negated, apply("---_", INPUT));
    }

    #[test]
    fn float() {
        const INPUT: &'static str = "42.42";
        let negated = format!("-{}", INPUT);
        assert_eq!(negated, apply("-_", INPUT));
        assert_eq!(INPUT, apply("--_", INPUT));
        assert_eq!(negated, apply("---_", INPUT));
    }
}

mod bang {
    use util::*;

    #[test]
    fn constant() {
        assert_eq!("false", eval("!true"));
        assert_eq!("true", eval("!!true"));
        assert_eq!("false", eval("!!!true"));
        assert_eq!("true", eval("!false"));
        assert_eq!("false", eval("!!false"));
        assert_eq!("true", eval("!!!false"));
    }

    #[test]
    fn input() {
        assert_eq!("false", apply("!_", "true"));
        assert_eq!("true", apply("!_", "false"));
    }
}
