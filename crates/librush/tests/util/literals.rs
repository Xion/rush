//! Module defining how to convert Rust objects into rush literals.
//! This allows to easily supply test data for rush expressions.

use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Display;


/// Type to hold a literal rush representation of a Rust object.
pub type Literal = String;

/// Defines how a Rust object can be converted to a rush literal.
pub trait ToLiteral {
    fn to_literal(&self) -> Literal;
}


/// Macro to implement ToLiteral using string formatting.
///
/// It is necessary because making a blanket implementation for each T: Display
/// introduces ambiguity when we also add the specialized cases for arrays & hashmaps.
macro_rules! impl_toliteral_via_format {
    ($t:ty) => {
        impl ToLiteral for $t {
            fn to_literal(&self) -> Literal { format!("{}", self) }
        }
    }
}

impl_toliteral_via_format!(String);
impl_toliteral_via_format!(i32);
impl_toliteral_via_format!(i64);
impl_toliteral_via_format!(f32);
impl_toliteral_via_format!(f64);

impl<T: ToString> ToLiteral for [T] {
    fn to_literal(&self) -> Literal {
        format!("[{}]", self.iter()
            .map(T::to_string).collect::<Vec<_>>().join(","))
    }
}

impl<K, V> ToLiteral for HashMap<K, V>
    where K: Display + Eq + Hash, V: Display
{
    fn to_literal(&self) -> Literal {
        format!("{{{}}}", self.iter()
            .map(|(ref k, ref v)| format!("{}:{}", k, v))
            .collect::<Vec<_>>().join(","))
    }
}
