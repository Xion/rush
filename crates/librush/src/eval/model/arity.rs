//! Types related to function arity, i.e. number of arguments it accepts.

use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::fmt;
use std::ops::{Add, Range, Sub};
use std::usize;

use conv::TryInto;
use conv::errors::Unrepresentable;

use super::value::Value;


/// Arguments to a function.
pub type Args = Vec<Value>;

/// Type for a number of arguments
/// (both expected by a function, and actually passed).
pub type ArgCount = usize;


/// Function arity (number of accepted arguments).
#[derive(Clone,Copy,Debug,PartialEq)]
pub enum Arity {
    /// Exact arity.
    /// Function requires the precise number of arguments, no more and no less.
    Exact(ArgCount),

    /// Minimum arity.
    /// Function requires at least that many arguments.
    Minimum(ArgCount),

    /// Arity range.
    /// Function requires at least as many arguments as the lower bound,
    /// but no more than the upper bound.
    Range(ArgCount, ArgCount),  // inclusive range!
}

impl Arity {
    #[inline]
    pub fn with_exact(argcount: ArgCount) -> Arity {
        Arity::Exact(argcount)
    }

    #[inline]
    pub fn with_minimum(argcount: ArgCount) -> Arity {
        Arity::Minimum(argcount)
    }

    #[inline]
    pub fn with_maximum(argcount: ArgCount) -> Arity {
        if argcount == 0 { Arity::Exact(0) }
        else             { Arity::Range(0, argcount) }
    }

    #[inline]
    pub fn with_range(min: ArgCount, max: ArgCount) -> Arity {
        assert!(min <= max,
            "Min. arity must not be greater than max. (got min={}, max={})",
            min, max);
        if min == max { Arity::Exact(min) }
        else          { Arity::Range(min, max) }
    }
}

impl Arity {
    #[inline]
    pub fn is_exact(&self) -> bool {
        match *self { Arity::Exact(..) => true, _ => false }
    }

    /// Returns the minimum number of arguments accepted by function with this arity.
    #[inline]
    pub fn minimum(&self) -> ArgCount {
        match *self {
            Arity::Exact(c) | Arity::Minimum(c) | Arity::Range(_, c) => c,
        }
    }

    /// Returns the maximum number of arguments accepted by function with this arity.
    #[inline]
    pub fn maximum(&self) -> ArgCount {
        match *self {
            Arity::Exact(c) | Arity::Range(_, c) => c,
            Arity::Minimum(_) => usize::MAX as ArgCount,
        }
    }

    /// Whether arity allows/accepts given argument count.
    /// This is equivalent to simple equality check: arity == argcount.
    #[inline]
    pub fn accepts(&self, argcount: ArgCount) -> bool {
        match *self {
            Arity::Exact(c) => argcount == c,
            Arity::Minimum(c) => argcount >= c,
            Arity::Range(a, b) => a <= argcount && argcount <= b,
        }
    }
}

impl fmt::Display for Arity {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Arity::Exact(c) => write!(fmt, "{}", c),
            Arity::Minimum(c) => write!(fmt, "{}+", c),
            Arity::Range(a, b) => write!(fmt, "{}-{}", a, b),
        }
    }
}


// Conversions

impl From<ArgCount> for Arity {
    #[inline]
    fn from(input: ArgCount) -> Self {
        Arity::Exact(input)
    }
}
impl TryInto<ArgCount> for Arity {
    type Err = Unrepresentable<Self>;

    #[inline]
    fn try_into(self) -> Result<ArgCount, Self::Err> {
        match self {
            Arity::Exact(c) => Ok(c),
            Arity::Range(a, b) if a == b => Ok(a),
            arity => Err(Unrepresentable(arity)),
        }
    }
}

impl From<Range<ArgCount>> for Arity {
    #[inline]
    fn from(input: Range<ArgCount>) -> Self {
        if input.len() > 0 {
            // Range<T> is half-open, Arity::Range is inclusive on both ends
            Arity::Range(input.start, input.end + 1)
        } else {
            Arity::Exact(input.start)
        }
    }
}
impl TryInto<Range<ArgCount>> for Arity {
    type Err = Unrepresentable<Self>;

    #[inline]
    fn try_into(self) -> Result<Range<ArgCount>, Self::Err> {
        match self {
            Arity::Exact(c) => Ok(Range{start: c, end: c}),
            Arity::Range(a, b) => Ok(Range{start: a, end: b + 1}),
            arity => Err(Unrepresentable(arity)),
        }
    }
}


// Ordering and equality

impl PartialOrd for Arity {
    /// Compare arities with each other.
    ///
    /// The ordering is only defined for those cases when the set of accepted
    /// argument counts either doesn't overlap at all, or overlaps completely.
    fn partial_cmp(&self, other: &Arity) -> Option<Ordering> {
        match (self, other) {
            (&Arity::Exact(c1), &Arity::Exact(c2)) => Some(c1.cmp(&c2)),

            (&Arity::Exact(c), &Arity::Range(a, b)) => {
                if c < a        { Some(Ordering::Less) }
                else if c > b   { Some(Ordering::Greater) }
                else            { None }
            },
            (&Arity::Range(a, b), &Arity::Exact(c)) => {
                if b < c        { Some(Ordering::Less) }
                else if a > c   { Some(Ordering::Greater) }
                else            { None }
            },

            (&Arity::Range(a1, b1), &Arity::Range(a2, b2)) => {
                if b1 < a2      { Some(Ordering::Less) }
                else if a1 > b2 { Some(Ordering::Greater) }
                else            { None }
            },

            _ => None,
        }
    }
}

impl PartialEq<ArgCount> for Arity {
    #[inline]
    fn eq(&self, count: &ArgCount) -> bool {
        if let Arity::Exact(c) = *self {
            return c == *count;
        }
        // other variants always returns false to maintain transitivity
        // with the derived PartialEq<Arity>.
        false
    }
}
impl PartialOrd<ArgCount> for Arity {
    /// Compare arity with an actual argument count.
    ///
    /// Result indicates whether the count satisfies the arity, or whether
    /// more/fewer arguments would be needed.
    #[inline]
    fn partial_cmp(&self, count: &ArgCount) -> Option<Ordering> {
        match *self {
            Arity::Exact(c) => c.partial_cmp(&count),
            Arity::Minimum(c) => Some(
                // Once the argument count is above minimum,
                // it is "equal" for all intents and purposes.
                if c <= *count { Ordering::Equal } else { Ordering::Greater }
            ),
            Arity::Range(a, b) => Some(
                // The argument count is "equal" if it is within range.
                if b < *count       { Ordering::Less }
                else if a > *count  { Ordering::Greater }
                else                { Ordering::Equal }
            ),
        }
    }
}


// Arithmetic

impl Add<ArgCount> for Arity {
    type Output = Arity;

    /// Adding a specific argument count to an arity,
    /// equivalent to introducing that many new argument slots to a function.
    #[inline]
    fn add(self, rhs: ArgCount) -> Self::Output {
        match self {
            Arity::Exact(c) => Arity::Exact(c + rhs),
            Arity::Minimum(c) => Arity::Minimum(c), // no change
            Arity::Range(a, b) => Arity::Range(a, b + rhs), // inc. upper bound
        }
    }
}

impl Sub<ArgCount> for Arity {
    type Output = Arity;

    /// Subtracting a specific argument count from an arity.
    /// Used to determine the new arity of a curried function.
    fn sub(self, rhs: ArgCount) -> Self::Output {
        match self {
            Arity::Exact(c) => {
                if c >= rhs {
                    return Arity::Exact(c - rhs);
                }
                panic!("underflow when subtracting from exact arity: {} - {} < 0",
                    c, rhs)
            },
            Arity::Minimum(c) => {
                if c > rhs {
                    return Arity::Minimum(c - rhs);
                } else if c == rhs {
                    return Arity::Exact(0);
                }
                panic!("underflow when subtracting from minimum arity: {} - {} < 0",
                    c, rhs)
            },
            Arity::Range(a, b) => {
                let span = b - a;
                if rhs < span {
                    return Arity::Range(a, b - rhs);
                } else if rhs == span {
                    return Arity::Exact(a);
                }
                panic!("underflow when subtracting from arity range: \
                    ({} - {}) - {} < 0", b, a, rhs)
            },
        }
    }
}
