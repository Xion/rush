# ap

Expression syntax overview

## Data types

* string (default if no explicit annotation/function/coercion is used)
* number: int or float
* booleans (true or false)

TODO(xion): hashmaps?? (for JSON maybe)

## Identifiers & values

Identifier starts with a letter (NOT underscore, because see below)
and can contain letters, numbers, and the underscore character.

If an identifier doesn't refer to a known function, it is treated as literal string.

Strings are surrounded with double quotes. \" to escape a quote, \\ to escape a backslash.

Integers are `[+-]?[1-9][0-9]*`.
Floats are additionally `[+-]?([0-9]\.)?[0-9]+(e$INTEGER)?` (i.e. regular & scientific notation).

## Special symbols

* `_` (underscore) -- Current item (without specifying its type).
* `_s` -- Current item as string.
* `_i` -- Current item as integer.
* `_f` -- Current item as float.

## Operators

* arithmetic: `+`, `-`, `*`, `/`; operate on numbers
* strings: `+` (concatentation), `*` (repeat), `%` (formatting)

TODO(xion): conditional/ternary operator?
TODO(xion): with arrays, split & join (e.g. `/` as split operator, `~` as join)

## Functions

Function names are identifiers.

Function invocation always involves parentheses around its argument list (even if empty).
Multiple arguments are separated with comma.

Anonymous functions are defined using `\`, an argument list, colon, and expression, e.g.:

    \: 42
    \x: x + 2
    \x,y: x + y

FIXME(xion): this won't work nicely with shell escaping (requires double backslash), probably need some other syntax ;/

`.` is the composition operator:

    abs . (\x: x + 2)   ===  \x: abs(x + 2)

TODO(xion): Haskell-like syntax for (partial application of) operator functions:
(+), (2+), (*5), etc.

## Reserved syntactic elements

All "special" characters (incl. braces, brackets, all symbols on the numeric row, and semicolon)
are reserved for possible future use. If string is to contain them, it must be surrounded by quotes.

Some possible future keywords are also reserved, e.g.: if else while for do.

## Execution

Depending on the type of the overall expression, the result of its execution is the following:

* if the type is a unary function, it is applied to the current item and its result
  is the output for the item
* if the type is a plain value, it is executed with `_` bound to current item
  and the result is the output for the item
* otherwise (e.g. function with more than one argument) it is a fatal error

Alternately, an expression such as `_ + 2` can be thought as a shorthand for `\x: x + 2`
(as long as typed versions `_` haven't been used).
