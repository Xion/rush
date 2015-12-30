extern crate getopts;
#[macro_use]
extern crate nom;

mod ast;
mod eval;
mod parse;


use std::env;
use std::io::{self, Read, Write, BufRead, BufReader, BufWriter};
use std::process::exit;

use getopts::Options;

use self::eval::{Eval, Context};
use self::parse::parse;


fn main() {
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
    if let Err(error) = apply(&expr, io::stdin(), io::stdout()) {
        // TODO(xion): user-friendly error messages
        writeln!(&mut io::stderr(), "{:?}", error).unwrap();
        exit(1);
    }
}


/// Print the instructions about invoking the program from the command line.
fn print_usage(program: &str, opts: Options) {
    println!("{}", opts.usage(&format!("Usage: {} [options]", program)));
}


/// Apply the expression to given input stream,
/// writing to the given output stream.
fn apply<R: Read, W: Write>(expr: &str, input: R, output: W) -> Result<(), io::Error> {
    let ast = try!(parse(expr)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e)));

    let reader = BufReader::new(input);
    let mut writer = BufWriter::new(output);
    let mut context = Context::new();

    for line in reader.lines() {
        let line = try!(line);
        context.set_string_var("_", &line);

        let result = try!(ast.eval(&context)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e)));
        try!(write!(writer, "{}\n", result));
    }
    Ok(())
}
