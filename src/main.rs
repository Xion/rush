//! Module with the entry point of the binary.

extern crate getopts;
#[macro_use]
extern crate log;

extern crate ap;


mod logging;


use std::env;
use std::io::{self, Write};
use std::process::exit;

use getopts::Options;


fn main() {
    logging::init().unwrap();

    let argv: Vec<String> = env::args().collect();
    let program = argv[0].clone();

    let mut options = Options::new();
    options.optflag("h", "help", "Show this usage message");

    let args = options.parse(&argv[1..]).unwrap();
    if args.opt_present("h") {
        print_usage(&program, options);
        return;
    }

    let expr = args.free.join(" ");
    if let Err(error) = ap::apply(&expr, io::stdin(), &mut io::stdout()) {
        error!("{:?}", error);
        exit(1);
    }
}


/// Print the instructions about invoking the program from the command line.
fn print_usage(program: &str, opts: Options) {
    writeln!(&mut io::stderr(), "{}",
             opts.usage(&format!("Usage: {} [options]", program))).unwrap();
}
