# rush

_Warning:_: Work in progress.

[![Build Status](https://img.shields.io/travis/Xion/rush.svg)](https://travis-ci.org/Xion/rush)
[![License](https://img.shields.io/github/license/Xion/rush.svg)](https://github.com/Xion/rush/blob/master/LICENSE)

Succinct & readable processing language for a Unix shell. Written in Rust.

    $ echo "Hello " | rh '_ + world'
    Hello world

## Requirements

Any Unix-like system should do.

## Usage

    rh [--input <MODE> | --string | --lines | --words | --chars | --bytes | --files]
        [--before <EXPRESSION>]
        [--after <EXPRESSION>]
        <EXPRESSION> [<EXPRESSION> ...]
    
    OPTIONS:
        -i, --input <MODE>
            Defines how the input should be treated when processed by EXPRESSION [values: string, lines, words, chars, bytes, files]
        -s, --string                 Apply the expression once to the whole input as single string
        -l, --lines                  Apply the expression to each line of input as string. This is the default
        -w, --words                  Apply the expression to each word in the input as string.
        -c, --chars                  Apply the expression to each character of input (treated as 1-character string).
        -b, --bytes                  Apply the expression to input bytes. The expression must take byte value as integer and return integer output.
        -f, --files                  Apply the expression to the content of each file (as string) whose path is given as a line of input
        -B, --before <EXPRESSION>
            Optional expression to evaluate before processing the input. The result of this expression is discarded but any side effects (assignments) will persist.
        -A, --after <EXPRESSION>
            Optional expression to evaluate after processing the input. If provided, only the result of this expression will be printed to standard output.
        -H, --help                   Prints help information
        -V, --version                Prints version information
    
    ARGS:
        <EXPRESSION>...
            Expression(s) to apply to input. When multiple expressions are given, the result of one is passed as input to the next one.

## Examples

### Strings

    $ echo 'Alice has a cat' | rh 'before("cat")' '_ + "dog"'
    Alice has a dog
    $ echo 'Alice has a cat' | rh 'after("Alice")' '"Bob" + _'
    Bob has a cat

# ROT13

    $ echo -n 'flap' | rh -c 'ord' '(_ - ord(a) + 13) % 26' '_ + ord(a)' chr | rh -s 'sub(/\s+/, "")'
    sync
    $ echo -n 'flap' | rh -s rot13
    sync

### CSV

    $ echo '1,2,3' | rh 'csv & map(int & (*2)) & csv'
    2,4,6

    $ rh 'csv' '{number: _[0], symbol: _[1], name: _[2], mass: _[3]}'  <./elements.csv
    {"mass":"1","name":"Hydrogen","number":"1","symbol":"H"}
    {"mass":"4","name":"Helium","number":"2","symbol":"He"}
    # etc.

## Contributing

You need a Rust toolchain (with Cargo) to build _rush_ itself.

Additionally, the Python-based [Invoke](http://pyinvoke.org) task runner is used for automation.
It is recommended you install it inside a Python virtualenv. e.g.:

    $ virtualenv ~/venv/rush && source ~/venv/rush/bin/activate
    $ pip install -r -requirements-dev.txt

Then you can use:

    $ inv

to build the binary & the library crate, and run their tests.

## License

_rush_ is licensed under the [GNU GPL v3](https://github.com/Xion/rush/blob/master/LICENSE) license.

Copyright © 2016, Karol Kuczmarski
