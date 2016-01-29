/// Utility module used by code that evaluates expressions.
///
/// Contains mostly macros that make type-safe function definitions
/// more concise.


// A few tips on how to read and/or modify these macros:
//
// * Rules that are more general (like those where argtypes != return type)
//   come BEFORE rules that are more specific (and in turn use those general
//   rules in their implementation).
//
//   This is because the macro parser scan them top-down and picks
//   the first match. The more general rules -- with additional tokens
//   over spcific rules, like `where` -- have thus to be excluded ASAP
//   if they don't apply. This way, the specific ones (which share much
//   of the syntax) have a chance to be matched.
//
// * Rules that generate variants operating on pointer types (e.g. &String)
//   have to be placed BEFORE those working with copied types (e.g. Integer).
//   For multi-argument variants, sequences of pointer type that are:
//     (1) longer -- e.g. (&a, &b) vs. (&a, b)
//     (2) closer to the beginning of argument list -- e.g. (&a, b) vs. (a, &b)
//   must come first.
//
//   This ensures the (limited) Rust's macro parser can correctly identify
//   the variant to use by just scanning one token at a time (it's LL(1)).
//   Unfortunately, this also means certain patterns are ambiguous,
//   like (&a, b, c) and (&a, b, &c), and only one of them can be supported.
//
// * For simplicity, rules that come later should always refer to
//   the MOST general previous rule in their implementation. This way
//   we won't have to trace multi-layered macro expansions when debugging,
//   also reducing the risk of running into further limitations of Rust
//   macro parser.


/// Evaluate a unary expression provided the argument match declared Value type.
///
/// Example usage:
///     eval1!(arg: Integer { arg + 42 });
///
macro_rules! eval1 {
    // (arg: &Foo) -> Bar where (pre()) { foo(arg) }
    (($x:ident: &$t:ident) -> $rt:ident where ($pre:expr) { $e:expr }) => {
        if let Value::$t(ref $x) = $x {
            if $pre {
                return Ok(Value::$rt($e));
            }
        }
    };
    // (arg: &Foo) -> Bar { foo(arg) }
    (($x:ident: &$t:ident) -> $rt:ident { $e:expr }) => {
        eval1!(($x: &$t) -> $rt where (true) { $e });
    };

    // (arg: Foo) -> Bar where (pre()) { foo(arg) }
    (($x:ident: $t:ident) -> $rt:ident where ($pre:expr) { $e:expr }) => {
        if let Value::$t($x) = $x {
            if $pre {
                return Ok(Value::$rt($e));
            }
        }
    };
    // (arg: Foo) -> Bar { foo(arg) }
    (($x:ident: $t:ident) -> $rt:ident { $e:expr }) => {
        eval1!(($x: $t) -> $rt where (true) { $e });
    };

    // arg : &Foo { foo(arg) }
    ($x:ident : &$t:ident { $e:expr }) => {
        eval1!(($x: &$t) -> $t where (true) { $e });
    };
    // arg : Foo { foo(arg) }
    ($x:ident : $t:ident { $e:expr }) => {
        eval1!(($x: $t) -> $t where (true) { $e });
    };
}


/// Evaluate a binary expression provided the argument match declared Value types.
///
/// Example usage:
///     eval2!(left, right: Integer { left + right });
///
macro_rules! eval2 {
    // (left: &Foo, right: &Bar) -> Baz where (pre()) { foo(left, right) }
    (($x:ident: &$t1:ident, $y:ident: &$t2:ident) -> $rt:ident where ($pre:expr) { $e:expr }) => {
        if let Value::$t1(ref $x) = $x {
            if let Value::$t2(ref $y) = $y {
                if $pre {
                    return Ok(Value::$rt($e));
                }
            }
        }
    };
    // (left: &Foo, right: &Bar) -> Baz { foo(left, right) }
    (($x:ident: &$t1:ident, $y:ident: &$t2:ident) -> $rt:ident { $e:expr }) => {
        eval2!(($x: &$t1, $y: &$t2) -> $rt where (true) { $e });
    };

    // (left: &Foo, right: Bar) -> Baz where (pre()) { foo(left, right) }
    (($x:ident: &$t1:ident, $y:ident: $t2:ident) -> $rt:ident where ($pre:expr) { $e:expr }) => {
        if let Value::$t1(ref $x) = $x {
            if let Value::$t2($y) = $y {
                if $pre {
                    return Ok(Value::$rt($e));
                }
            }
        }
    };
    // (left: &Foo, right: Bar) -> Baz { foo(left, right) }
    (($x:ident: &$t1:ident, $y:ident: $t2:ident) -> $rt:ident { $e:expr }) => {
        eval2!(($x: &$t1, $y: $t2) -> $rt where (true) { $e });
    };

    // (left: Foo, right: &Bar) -> Baz where (pre()) { foo(left, right) }
    (($x:ident: $t1:ident, $y:ident: &$t2:ident) -> $rt:ident where ($pre:expr) { $e:expr }) => {
        if let Value::$t1($x) = $x {
            if let Value::$t2(ref $y) = $y {
                if $pre {
                    return Ok(Value::$rt($e));
                }
            }
        }
    };
    // (left: Foo, right: &Bar)-> Baz { foo(left, right) }
    (($x:ident: $t1:ident, $y:ident: &$t2:ident) -> $rt:ident { $e:expr }) => {
        eval2!(($x: $t1, $y: &$t2) -> $rt where (true) { $e });
    };

    // (left: Foo, right: Bar) -> Baz where (pre()) { foo(left, right) }
    (($x:ident: $t1:ident, $y:ident: $t2:ident) -> $rt:ident where ($pre:expr) { $e:expr }) => {
        if let Value::$t1($x) = $x {
            if let Value::$t2($y) = *$y {
                if $pre {
                    return Ok(Value::$rt($e));
                }
            }
        }
    };
    // (left: Foo, right: Bar) -> Baz { foo(left, right) }
    (($x:ident: $t1:ident, $y:ident: $t2:ident) -> $rt:ident { $e:expr }) => {
        eval2!(($x: $t1, $y: $t2) -> $rt where (true) { $e });
    };

    // left, right : &Foo { foo(left, right) }
    ($x:ident, $y:ident : &$t:ident { $e:expr }) => {
        eval2!(($x: &$t, $y: &$t) -> $t where (true) { $e });
    };
    // left, right : Foo { foo(left, right) }
    ($x:ident, $y:ident : $t:ident { $e:expr }) => {
        eval2!(($x: $t, $y: $t) -> $t where (true) { $e });
    };
}


// TODO(xion): define eval3!(...)
