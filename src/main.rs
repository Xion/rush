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
        match ap::parse(expr) {
            Ok(ast) => println!("{:?}", ast),
            Err(error) => { error!("{:?}", error); exit(1); },
        }
    } else {
        // apply the expression either to every line separately,
        // or the whole input as an array of lines
        // TODO(xion): implement "words", where each line is an array of words
        let apply: fn(_, _, _) -> _ =
            if args.is_present("all") { ap::reduce } else { ap:: map };
        if let Err(error) = apply(expr, io::stdin(), &mut io::stdout()) {
            error!("{:?}", error);
            exit(1);
        }
    }
}

