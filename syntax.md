# rush

Expression syntax overview

TODO(xion): either move this all into README or to dedicated documentation page

## Data types

* string (default if no explicit annotation/function/coercion is used)
* number: int or float
* booleans (true or false)
* arrays
* objects (hashmaps string -> value)

## Identifiers & values

Identifier starts with a letter (NOT underscore, because see below)
and can contain letters, numbers, and the underscore character.

If an identifier doesn't refer to a known function, it is treated as literal string.

Strings are surrounded with double quotes. \" to escape a quote, \\ to escape a backslash.

Integers are `[+-]?[1-9][0-9]*`.
Floats are additionally `[+-]?([0-9]\.)?[0-9]+(e$INTEGER)?` (i.e. regular & scientific notation).

## Special symbols

* `_` (underscore) -- Current item.

## Operators

* arithmetic: `+`, `-`, `*`, `/`; operate on numbers
* strings: `+` (concatentation), `*` (repeat), `%` (formatting), `/` (split)
* ternary operator: `?:`

## Functions

Function names are identifiers.

Function invocation always involves parentheses around its argument list (even if empty).
Multiple arguments are separated with comma.

Anonymous functions are defined using `|` (pipe), an argument list, another pipe, and expression, e.g.:

    || 42
    |x| x + 2
    |x,y| x + y

`&` is the "reverse function composition" (piping) operator:

    int & abs & |x| x / 2  ===  |x| abs(int(x)) / 2

Functions are automatically curried when given fewer than minimum number of arguments:

    split(",")  ===  |s| split(",", s)

These features can of course be combined:

    $ echo '1,2,3' | ap 'split(",") & map(int & |x| x * 2) & join(",")'
    2,4,6

There is also a Haskell-like syntax for (partial application of) operator functions
`(+)`, `(2+)`, `(*5)`, etc.

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

Alternately, an expression such as `_ + 2` can be thought as a shorthand for `|x| x + 2`
(as long as typed versions `_` haven't been used).
