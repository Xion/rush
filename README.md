# ap

(Experimental, work in progress, do not use, etc.)

[![Build Status](https://img.shields.io/travis/Xion/ap.svg)](https://travis-ci.org/Xion/ap)
[![License](https://img.shields.io/github/license/Xion/ap.svg)](https://github.com/Xion/ap/blob/master/LICENSE)

Succinct & readable processing language for a Unix shell. Written in Rust.

    $ echo "Hello " | ap '_ + world'
    Hello world

    $ echo '1,2,3' | ap 'split(",") & map(int & (*2)) & join(", ")'
    2, 4, 6

## Requirements

WIP

## Examples

WIP

## Usage

WIP

## Contributing

WIP

## License

_ap_ is licensed under the [GNU GPL v3](https://github.com/Xion/ap/blob/master/LICENSE) license.
Copyright © 2016, Karol Kuczmarski
