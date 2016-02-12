//! Module with the entry point of the binary.

extern crate clap;
#[macro_use]
extern crate log;

extern crate ap;


mod args;
mod logging;


use std::io;
use std::process::exit;


fn main() {
    logging::init().unwrap();

    let args = args::parse();
    let expr = args.value_of("expr").unwrap();

    if args.is_present("parse") {
        print_ast(expr);
        return;
    }

    // apply the expression either to every line separately,
    // or the whole input as an array of lines
    // TODO(xion): implement "words", where each line is an array of words
    let apply: fn(_, _, _) -> _ =
        if args.is_present("all") { ap::apply_lines } else { ap:: map_lines };
    if let Err(error) = apply(expr, io::stdin(), &mut io::stdout()) {
        error!("{:?}", error);
        exit(1);
    }
}


/// Print the AST for given expression to stdout.
fn print_ast(expr: &str) {
    match ap::parse(expr) {
        Ok(ast) => println!("{:?}", ast),
        Err(error) => { error!("{:?}", error); exit(1); },
    }
}
