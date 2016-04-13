//! Types related to function arity, i.e. number of arguments it accepts.

use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::fmt;
use std::ops::{Add, Sub};

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
    Range(ArgCount, ArgCount),
}

impl Arity {
    #[inline(always)]
    pub fn is_exact(&self) -> bool {
        match *self { Arity::Exact(..) => true, _ => false }
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

impl PartialOrd for Arity {
    /// Compare arities with each other.
    /// The ordering is only defined for exact arities.
    fn partial_cmp(&self, other: &Arity) -> Option<Ordering> {
        match *self {
            Arity::Exact(c1) => {
                if let Arity::Exact(c2) = *other {
                    return Some(c1.cmp(&c2));
                }
                None
            },
            // TODO(xion): ordering can be defined for any combination of Range & Exact
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
                if *count >= c { Ordering::Equal } else { Ordering::Less }
            ),
            Arity::Range(a, b) => Some(
                // The argument count is "equal" if it is within range.
                if *count < a       { Ordering::Less }
                else if *count > b  { Ordering::Greater }
                else                { Ordering::Equal }
            ),
        }
    }
}

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
