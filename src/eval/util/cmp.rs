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
