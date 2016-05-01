//! Custom traits related to ordering and comparison.

use std::cmp::Ordering;


/// Trait for values that can be optionally compared for a sort-order.
///
/// Unlike PartialOrd, this one treats the unspecified ordering of values
/// as an errorneous condition. As such, it is more similar to Ord,
/// and also analogous to how TryFrom and TryInto traits from the conv crate
/// relate to the standard From and Into traits.
pub trait TryOrd<Rhs: ?Sized = Self> {
    type Err;
    fn try_cmp(&self, other: &Rhs) -> Result<Ordering, Self::Err>;

    fn try_lt(&self, other: &Rhs) -> Result<bool, Self::Err> {
        self.try_cmp(other).map(|o| o == Ordering::Less)
    }
    fn try_le(&self, other: &Rhs) -> Result<bool, Self::Err> {
        self.try_cmp(other).map(|o| o == Ordering::Less || o == Ordering::Equal)
    }
    fn try_gt(&self, other: &Rhs) -> Result<bool, Self::Err> {
        self.try_cmp(other).map(|o| o == Ordering::Greater)
    }
    fn try_ge(&self, other: &Rhs) -> Result<bool, Self::Err> {
        self.try_cmp(other).map(|o| o == Ordering::Greater || o == Ordering::Equal)
    }
}

/// Trait for equality comparisons that may fail.
///
/// Unlike Eq & PartialEq, this one treats the situation where two values cannot
/// be compared as an error. As such, it is somewhat analogous to how
/// TryFrom and TryInto traits from the conv crate relate to the standard
/// From and Into traits.
pub trait TryEq<Rhs: ?Sized = Self> {
    type Err;
    fn try_eq(&self, other: &Rhs) -> Result<bool, Self::Err>;

    fn try_ne(&self, other: &Rhs) -> Result<bool, Self::Err> {
        self.try_eq(other).map(|b| !b)
    }
}


// Macros for implementing PartialX from TryX

// These macros are necessary, because Rust only allows traits defined with current crate
// to be impl'd for template params. This makes the following generic impl illegal:
//
//     impl<T, Rhs> PartialOrd<Rhs> for T where T: TryOrd<Rhs> { ... }
//
// The macros allow to at least reduce the boilerplate for creating those impls
// for a particular type to minimum.

macro_rules! impl_partialord_for_tryord (
    ($t:ty) => {
        impl ::std::cmp::PartialOrd for $t {
            fn partial_cmp(&self, other: &$t) -> Option<::std::cmp::Ordering> {
                self.try_cmp(other).ok()
            }
        }
    };
);

macro_rules! impl_partialeq_for_tryeq (
    ($t:ty) => {
        impl ::std::cmp::PartialEq for $t {
            fn eq(&self, other: &$t) -> bool {
                self.try_eq(other).unwrap_or(false)
            }
        }
    };
);
