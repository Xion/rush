//! Test crate.
//!
//! Note that the actual test cases are in the `tests` submodule.
//!
//! They can't be in the tests/ root, because `cargo test` executes every
//! root *.rs file as a separate binary, which would e.g. require repeating
//! of the `extern crate` declarations and dealing with unused code warnings
//! within the `util` module.

#[macro_use]
extern crate maplit;
extern crate rustc_serialize;

extern crate rush;


#[macro_use]
mod util;
mod tests;
