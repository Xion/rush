//! Module with the entry point of the binary.

extern crate case;
extern crate clap;
extern crate conv;
#[macro_use]
extern crate log;

extern crate rush;


mod args;
mod logging;


use std::io;
use std::process::exit;

use args::InputMode;


fn main() {
    logging::init().unwrap();

    let opts = args::parse();
    let expr = opts.expression;

    if opts.input_mode.is_none() {
        print_ast(&expr);
        return;
    }

    // choose a function to process the input with, depending on flags
    let apply: fn(_, _, _) -> _ = match opts.input_mode.unwrap() {
        InputMode::String => rush::apply_string,
        InputMode::Lines => rush::map_lines,
        InputMode::Chars => rush::map_chars,
    };
    if let Err(error) = apply(&expr, io::stdin(), &mut io::stdout()) {
        error!("{:?}", error);
        exit(1);
    }
}


/// Print the AST for given expression to stdout.
fn print_ast(expr: &str) {
    debug!("Printing the AST of:  {}", expr);
    match rush::parse(expr) {
        Ok(ast) => println!("{:?}", ast),
        Err(error) => { error!("{:?}", error); exit(1); },
    }
}
