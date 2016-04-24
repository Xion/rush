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

    rh [--input <MODE> | --string | --lines | --words | --chars | --bytes] <EXPRESSION> [<EXPRESSION> ...]

    OPTIONS:
        -i, --input <MODE>
            Defines how the input should be treated when processed by EXPRESSION
            [values: string, lines, words, chars, bytes]
        -s, --string     Apply the expression once to the whole input as single string
        -l, --lines      Apply the expression to each line of input as string. This is the default
        -w, --words      Apply the expression to each word in the input as string.
        -c, --chars      Apply the expression to each character of input (which is treated as 1-character string).
        -b, --bytes      Apply the expression to input bytes.
                         The expression must take byte value as integer and return integer output.s

    ARGS:
        <EXPRESSION>...
            Expression(s) to apply to input.
            When multiple expressions are given, the result of one is passed as input to the next one.

## Examples

### Strings

    $ echo 'Alice has a cat' | rh 'before("cat")' '_ + "dog"'
    Alice has a dog

    # ROT13
    $ echo -n 'flap' | rh -c 'ord' '(_ - ord(a) + 13) % 26' '_ + ord(a)' chr | rh -s 'sub(/\s+/, "")'
    sync

### CSV

    $ echo '1,2,3' | rh 'csv & map(int & (*2)) & csv'
    2,4,6

    $ rh 'csv' '{number: _[0], symbol: _[1], name: _[2], mass: _[3]}'  <./elements.csv
    {"mass":"1","name":"Hydrogen","number":"1","symbol":"H"}
    {"mass":"4","name":"Helium","number":"2","symbol":"He"}
    # etc.


## Contributing

WIP

## License

_rush_ is licensed under the [GNU GPL v3](https://github.com/Xion/rush/blob/master/LICENSE) license.

Copyright © 2016, Karol Kuczmarski
