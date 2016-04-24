# rush

_Warning:_: Work in progress.

[![Build Status](https://img.shields.io/travis/Xion/rush.svg)](https://travis-ci.org/Xion/rush)
[![License](https://img.shields.io/github/license/Xion/rush.svg)](https://github.com/Xion/rush/blob/master/LICENSE)

Succinct & readable processing language for a Unix shell. Written in Rust.

    $ echo "Hello " | rh '_ + world'
    Hello world

## Requirements

Any Unix-like system should do.

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

## Usage

WIP

## Contributing

WIP

## License

_rush_ is licensed under the [GNU GPL v3](https://github.com/Xion/rush/blob/master/LICENSE) license.

Copyright © 2016, Karol Kuczmarski
