//! Module for handling command line arguments to the application.

use std::env;
use std::ffi::OsString;
use std::iter::IntoIterator;

use clap::{self, AppSettings, Arg, ArgSettings, ArgGroup, ArgMatches};
use conv::TryFrom;
use conv::errors::Unrepresentable;


/// Structure holding the options parsed from command line.
#[derive(Clone)]
pub struct Options {
    /// How to interpret the input (if anyhow).
    pub input_mode: Option<InputMode>,
    /// Optional expression to execute right before processing the input.
    pub before: Option<String>,
    /// Exoressions to process the input through, one after another.
    /// The result of the final expression will be result of the whole computation
    /// (unless `after` is defined below).
    pub expressions: Vec<String>,
    /// Optional expression to execute right after processing the input.
    /// If defined, only its result will be printed as output.
    pub after: Option<String>,
}

impl<'a> From<ArgMatches<'a>> for Options {
    fn from(matches: ArgMatches<'a>) -> Self {
        Options{
            before: matches.value_of(OPT_BEFORE).map(String::from),
            expressions: matches
                             .values_of(ARG_EXPRESSION).unwrap()
                             .map(String::from).collect(),
            after: matches.value_of(OPT_AFTER).map(String::from),
            input_mode: if matches.is_present(OPT_PARSE) { None }
                        else { Some(InputMode::from(matches)) },
        }
    }
}

/// Defines possible options as to how the program's input
/// may be processed by the expression(s).
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum InputMode {
    String,
    Lines,
    Words,
    Chars,
    Bytes,
    Files,
}

impl InputMode {
    fn description(&self) -> &str {
        match *self {
            InputMode::String => "whole input as string",
            InputMode::Lines => "line by line",
            InputMode::Words => "word by word",
            InputMode::Chars => "character by character",
            InputMode::Bytes => "byte by byte",
            InputMode::Files => "file by file",
        }
    }
}

impl Default for InputMode {
    fn default() -> Self { InputMode::Lines }
}

impl<'s> TryFrom<&'s str> for InputMode {
    type Err = Unrepresentable<String>;

    fn try_from(mode: &'s str) -> Result<Self, Self::Err> {
        match mode {
            "string" => Ok(InputMode::String),
            "lines" => Ok(InputMode::Lines),
            "words" => Ok(InputMode::Words),
            "chars" => Ok(InputMode::Chars),
            "bytes" => Ok(InputMode::Bytes),
            "files" => Ok(InputMode::Files),
            _ => Err(Unrepresentable(mode.to_owned())),
        }
    }
}

impl<'a> From<ArgMatches<'a>> for InputMode {
    fn from(matches: ArgMatches<'a>) -> Self {
        // decide the input mode based either on --input flag's value
        // or dedicated flags, like --string
        for &mode in INPUT_MODES {
            if matches.value_of(OPT_INPUT_MODE) == Some(mode) || matches.is_present(mode) {
                return InputMode::try_from(mode).unwrap();
            }
        }
        let default = InputMode::default();
        info!("Using default processing mode ({})", default.description());
        default
    }
}


/// Parse command line arguments and return matches' object.
#[inline]
pub fn parse() -> Options {
    parse_from_argv(env::args_os())
}

/// Parse application options from given array of arguments
/// (*all* arguments, including binary name).
#[inline]
pub fn parse_from_argv<I, T>(argv: I) -> Options
    where I: IntoIterator<Item=T>, T: Into<OsString>
{
    let matches = create_parser().get_matches_from(argv);
    Options::from(matches)
}


// Parser configuration

/// Type of the argument parser object
/// (which is called an "App" in clap's silly nomenclature).
type Parser<'p> = clap::App<'p, 'p>;


const APP_NAME: &'static str = "rush";
const APP_DESC: &'static str = "Succinct & readable processing language";

const USAGE: &'static str = concat!("rush", " [",
    "--input <MODE>", " | ",
    "--string | --lines | --words | --chars | --bytes | --files",
    "] ",
    "[--before <EXPRESSION>] ", "[--after <EXPRESSION>] ",
    "<EXPRESSION> ", "[<EXPRESSION> ...]");

const OPT_INPUT_MODE: &'static str = "mode";
const INPUT_MODES: &'static [&'static str] = &[
    "string", "lines", "words", "chars", "bytes", "files",
];
const OPT_PARSE: &'static str = "parse";

const OPT_BEFORE: &'static str = "before";
const ARG_EXPRESSION: &'static str = "expr";
const OPT_AFTER: &'static str = "after";


/// Creates the argument parser.
fn create_parser<'p>() -> Parser<'p> {
    let mut parser = Parser::new(APP_NAME);
    if let Some(version) = option_env!("CARGO_PKG_VERSION") {
        parser = parser.version(version);
    }
    parser
        .about(APP_DESC)
        .usage(USAGE)

        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::UnifiedHelpMessage)
        .setting(AppSettings::DeriveDisplayOrder)

        .group(ArgGroup::with_name("input_group")
            .arg(OPT_INPUT_MODE)
            .args(INPUT_MODES))
        .arg(Arg::with_name(OPT_INPUT_MODE)
            .short("i").long("input")
            .takes_value(true)
            .possible_values(INPUT_MODES)
            .help("Defines how the input should be treated \
                   when processed by EXPRESSION").next_line_help(true)
            .value_name("MODE"))
        .arg(Arg::with_name("string")
            .short("s").long("string")
            .help("Apply the expression once to the whole input as single string"))
        .arg(Arg::with_name("lines")
            .short("l").long("lines")
            .help("Apply the expression to each line of input as string. This is the default"))
        .arg(Arg::with_name("words")
            .short("w").long("words")
            .help("Apply the expression to each word in the input as string."))
        .arg(Arg::with_name("chars")
            .short("c").long("chars")
            .help("Apply the expression to each character of input \
                   (treated as 1-character string)."))
        .arg(Arg::with_name("bytes")
            .short("b").long("bytes")
            .help("Apply the expression to input bytes. \
                   The expression must take byte value as integer and return integer output."))
        .arg(Arg::with_name("files")
            .short("f").long("files")
            .help("Apply the expression to the content of each file (as string) \
                   whose path is given as a line of input"))

        // TODO: add a -0 (zero) option that changes the separator byte from \n to \0;
        // this shall apply to --lines and --files input modes (as well as their output)

        .arg(Arg::with_name(OPT_BEFORE)
            .short("B").long("before")
            .takes_value(true)
            .help("Optional expression to evaluate before processing the input. \
                   The result of this expression is discarded but any side effects (assignments) \
                   will persist.").next_line_help(true)
            .value_name("EXPRESSION"))
        .arg(Arg::with_name(ARG_EXPRESSION)
            .required(true)
            .multiple(true)
            .use_delimiter(false)  // don't interpret comma as arg separator
            .help("Expression(s) to apply to input. \
                   When multiple expressions are given, the result of one is \
                   passed as input to the next one.").next_line_help(true)
            .value_name("EXPRESSION"))
        .arg(Arg::with_name(OPT_AFTER)
            .short("A").long("after")
            .takes_value(true)
            .help("Optional expression to evaluate after processing the input. \
                   If provided, only the result of this expression will be printed \
                   to standard output.").next_line_help(true)
            .value_name("EXPRESSION"))

        .arg(Arg::with_name(OPT_PARSE)
            .set(ArgSettings::Hidden)
            .conflicts_with("input_group")
            .short("p").long("parse")
            .help("Only parse the expressions, printing their ASTs"))

        .help_short("H")
        .version_short("V")
}



/// Tests verifying the soundness of the above definition.
#[cfg(test)]
mod tests {
    use case::CaseExt;
    use conv::TryFrom;
    use super::{APP_NAME, INPUT_MODES, USAGE, InputMode};

    #[test]
    fn input_modes_are_consistent() {
        for &mode in INPUT_MODES {
            assert!(InputMode::try_from(mode).is_ok(),
                "Undefined InputMode variant: {}", mode.to_capitalized());
        }
    }

    #[test]
    fn usage_starts_with_app_name() {
        let prefix = APP_NAME.to_owned() + " ";
        assert!(USAGE.starts_with(&prefix), "Usage string must start with APP_NAME");
    }

    #[test]
    fn usage_contains_all_input_modes() {
        for mode in INPUT_MODES {
            let flag = "--".to_owned() + mode;
            assert!(USAGE.contains(&flag),
                "Input mode '{}' is missing from usage string", mode);
        }
    }
}
