//! Tests for comparison operators.


mod less {
    use util::*;

    #[test]
    fn constants() {
        assert_eval_true("1 < 2");
        assert_eval_true("-5 < 0");
        assert_eval_true("1.5 < 2");
        assert_eval_true("8 < 10.0");
        assert_eval_true("-3.14 < 3.14");
        assert_eval_false("1 < 1");
        assert_eval_false("0 < -10");
        assert_eval_error("0 < foo");
        assert_eval_error("foo < 42");
        assert_eval_error("bar < true");
        assert_eval_error("[] < []");
        assert_eval_error("{} < {}");
    }

    // TODO(xion): inputs
}

mod less_or_equal {
    use util::*;

    #[test]
    fn constants() {
        assert_eval_true("1 <= 2");
        assert_eval_true("-5 <= 0");
        assert_eval_true("1.5 <= 2");
        assert_eval_true("8 <= 10.0");
        assert_eval_true("-3.14 <= 3.14");
        assert_eval_true("1 <= 1");
        assert_eval_false("0 <= -10");
        assert_eval_false("-8 <= -12");
        assert_eval_error("0 <= foo");
        assert_eval_error("foo <= 42");
        assert_eval_error("bar <= true");
        assert_eval_error("[] <= []");
        assert_eval_error("{} <= {}");
    }

    // TODO(xion): inputs
}

mod greater {
    use util::*;

    #[test]
    fn constants() {
        assert_eval_true("2 > 1");
        assert_eval_true("0 > -5");
        assert_eval_true("2 > 1.5");
        assert_eval_true("10.0 > 8");
        assert_eval_true("3.14 > -3.14");
        assert_eval_false("1 > 1");
        assert_eval_false("-10 > 0");
        assert_eval_false("-12 > -8");
        assert_eval_error("0 > foo");
        assert_eval_error("foo > 42");
        assert_eval_error("bar > true");
        assert_eval_error("[] > []");
        assert_eval_error("{} > {}");
    }
    // TODO(xion): inputs
}

mod greater_or_equal {
    use util::*;

    #[test]
    fn constants() {
        assert_eval_true("2 >= 1");
        assert_eval_true("0 >= -5");
        assert_eval_true("2 >= 1.5");
        assert_eval_true("10.0 >= 8");
        assert_eval_true("3.14 >= -3.14");
        assert_eval_true("1 >= 1");
        assert_eval_false("-10 >= 0");
        assert_eval_false("-12 >= -8");
        assert_eval_error("0 >= foo");
        assert_eval_error("foo >= 42");
        assert_eval_error("bar >= true");
        assert_eval_error("[] >= []");
        assert_eval_error("{} >= {}");
    }
    // TODO(xion): inputs
}

mod equal {
    use util::*;

    #[test]
    fn constants() {
        assert_eval_false("2 == 1");
        assert_eval_false("0 == -5");
        assert_eval_false("2 == 1.5");
        assert_eval_false("10.0 == 8");
        assert_eval_false("3.14 == -3.14");
        assert_eval_false("-10 == 0");
        assert_eval_false("-12 == -8");
        assert_eval_true("1 == 1");
        assert_eval_true("2.0 == 2");
        assert_eval_true("3.0 == 3.0");
        assert_eval_true("4 == 4.0");
        assert_eval_true("[] == []");
        assert_eval_true("{} == {}");
        assert_eval_error("0 == foo");
        assert_eval_error("foo == 42");
        assert_eval_error("bar == true");
    }
    // TODO(xion): inputs
}

mod not_equal {
    use util::*;

    #[test]
    fn constants() {
        assert_eval_true("2 != 1");
        assert_eval_true("0 != -5");
        assert_eval_true("2 != 1.5");
        assert_eval_true("10.0 != 8");
        assert_eval_true("3.14 != -3.14");
        assert_eval_true("-10 != 0");
        assert_eval_true("-12 != -8");
        assert_eval_false("1 != 1");
        assert_eval_false("2.0 != 2");
        assert_eval_false("3.0 != 3.0");
        assert_eval_false("4 != 4.0");
        assert_eval_false("[] != []");
        assert_eval_false("{} != {}");
        assert_eval_error("0 != foo");
        assert_eval_error("foo != 42");
        assert_eval_error("bar != true");
    }
    // TODO(xion): inputs
}
