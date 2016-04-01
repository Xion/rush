//! Module for handling command line arugments to the application.

use std::env;
use std::ffi::OsString;
use std::iter::IntoIterator;

use clap::{self, AppSettings, Arg, ArgSettings, ArgGroup, ArgMatches};


// Type of the argument parser object.
pub type Parser<'p> = clap::App<'p, 'p>;


const APP_NAME: &'static str = "rush";
const APP_DESC: &'static str = "Succint & readable processing language";
const APP_AUTHOR: &'static str = "Karol Kuczmarski";

const INPUT_MODES: &'static [&'static str] = &["string", "lines"];


/// Parse command line arguments and return matches' object.
#[inline(always)]
pub fn parse<'a>() -> ArgMatches<'a> {
    parse_from_argv(env::args_os())
}

/// Parse application arguments given array of arguments
/// (*all* arguments, including binary name).
#[inline(always)]
pub fn parse_from_argv<'a, I, T>(argv: I) -> ArgMatches<'a>
    where I: IntoIterator<Item=T>, T: Into<OsString>
{
    create_parser().get_matches_from(argv)
}


/// Creates the argument parser
/// (which is called an "App" in clap's silly nomenclature).
fn create_parser<'p>() -> Parser<'p> {
    let mut parser = clap::App::new(APP_NAME);
    if let Some(version) = option_env!("CARGO_PKG_VERSION") {
        parser = parser.version(version);
    }
    parser
        .about(APP_DESC)
        .author(APP_AUTHOR)

        .setting(AppSettings::ArgRequiredElseHelp)

        // Usage has to be given explicitly because otherwise it will print
        // an incorrectly used flag twice -_-
        // TODO(xion): file an issue against clap about this
        .usage("rush [--input <MODE> | --string | --lines] <EXPRESSION>")

        // TODO(xion): implement missing input modes:
        // * (maybe) words - each word evaluated separately
        // * chars - each character separately (as one-character string)
        // * (maybe) bytes - each byte separately (as integer)
        .group(ArgGroup::with_name("input_group")
            .arg("mode")
            .args(INPUT_MODES))
        .arg(Arg::with_name("mode")
            .short("i").long("input")
            .takes_value(true)
            .possible_values(INPUT_MODES)
            .help("Defines how the input should be treated when processed by EXPRESSION")
            .value_name("MODE"))
        .arg(Arg::with_name("string")
            .short("s").long("string")
            .help("Apply the expression once to the whole input as single string"))
        .arg(Arg::with_name("lines")
            .short("l").long("lines")
            .help("Apply the expression to each line of input as string. This is the default"))

        .arg(Arg::with_name("parse")
            .set(ArgSettings::Hidden)
            .conflicts_with("input_group")
            .short("p").long("parse")
            .help("Only parse the expression, printing its AST"))

        .arg(Arg::with_name("expr")
            .use_delimiter(false)  // don't interpret comma as arg separator
            .help("Expression to apply to input")
            .value_name("EXPRESSION")
            .required(true))

        .help_short("h")
        .version_short("v")
}
