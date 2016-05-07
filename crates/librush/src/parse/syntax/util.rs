//! Utility macros used by the syntax definition.


/// Make the underlying parser assume UTF8-encoded input
/// and output String objects.
macro_rules! string (
    ($i:expr, $submac:ident!( $($args:tt)* )) => ({
        use std::str::from_utf8;
        map!($i, map_res!($submac!($($args)*), from_utf8), String::from)
    });
    ($i:expr, $f:expr) => (
        string!($i, call!($f));
    );
);


/// Make the underlying parser optional,
/// but unlike opt! it is treating incomplete input as parse error.
macro_rules! maybe (
    ($i:expr, $submac:ident!( $($args:tt)* )) => (
        opt!($i, complete!($submac!($($args)*)));
    );
    ($i:expr, $f:expr) => (
        maybe!($i, call!($f));
    );
);

/// Parse a sequence that matches the first parser followed by the second parser.
/// Return consumed input as the result (like recognize! does).
macro_rules! seq (
    // TODO(xion): generalize to arbitrary number of arguments (using chain!())
    ($i:expr, $submac:ident!( $($args:tt)* ), $submac2:ident!( $($args2:tt)* )) => ({
        // Unfortunately, this cannot be implemented straightforwardly as:
        //     recognize!($i, pair!($submac!($($args)*), $submac2!($($args2)*)));
        // because Rust compiler fails to carry out the type inference correctly
        // in the generated code.
        //
        // Below is therefore essentially a rewrite of nom's recognize!() macro.
        use nom::{HexDisplay, IResult};
        match pair!($i, $submac!($($args)*), $submac2!($($args2)*)) {
            IResult::Error(a)      => IResult::Error(a),
            IResult::Incomplete(i) => IResult::Incomplete(i),
            IResult::Done(i, _) => {
                let index = ($i).offset(i);
                IResult::Done(i, &($i)[..index])
            },
        }
    });
    ($i:expr, $submac:ident!( $($args:tt)* ), $g:expr) => (
        seq!($i, $submac!($($args)*), call!($g));
    );
    ($i:expr, $f:expr, $submac:ident!( $($args:tt)* )) => (
        seq!($i, call!($f), $submac!($($args)*));
    );
    ($i:expr, $f:expr, $g:expr) => (
        seq!($i, call!($f), call!($g));
    );
);


/// Parses values that are optionally surrounded by arbitrary number of
/// any of the whitespace characters.
macro_rules! multispaced (
    ($i:expr, $submac:ident!( $($args:tt)* )) => ({
        use nom::multispace;
        delimited!($i, opt!(multispace), $submac!($($args)*), opt!(multispace))
    });
    ($i:expr, $f:expr) => (
        multispaced!($i, call!($f));
    );
);

/// Matches exactly one character from the specified string.
/// This is like one_of!, but returns the matched char as &[u8] (assumming UTF8).
macro_rules! char_of (
    ($i:expr, $inp:expr) => (
        map!($i, one_of!($inp), |c: char| &$i[0..c.len_utf8()]);
    );
);
