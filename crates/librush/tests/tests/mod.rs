//! Module with actual tests.

mod api;
mod constants;
mod operators;
mod trailers;


/// These are miscellaneous tests that hasn't been moved to a dedicated submodule yet.
/// They are mostly to prevent regressions of various bugs.
mod misc {
    use rush::{Context, eval, Value};

    /// Test that assigning a lambda to a name works.
    #[test]
    fn assign_lambda_variable() {
        let mut context = Context::new();
        eval("inc = |x| x + 1", &mut context).unwrap();
        assert_eq!(Value::Integer(43), eval("inc(42)", &mut context).unwrap());
    }
}
