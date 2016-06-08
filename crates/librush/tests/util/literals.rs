//! Module defining how to convert Rust objects into rush literals.
//! This allows to easily supply test data for rush expressions.

use std::collections::HashMap;
use std::hash::Hash;

use regex::Regex;


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

impl_toliteral_via_format!(bool);
impl_toliteral_via_format!(i32);
impl_toliteral_via_format!(i64);
impl_toliteral_via_format!(u32);
impl_toliteral_via_format!(u64);
impl_toliteral_via_format!(f32);
impl_toliteral_via_format!(f64);

impl<'s> ToLiteral for &'s str {
    fn to_literal(&self) -> Literal {
        format!("\"{}\"", self.to_owned()
            // TODO: handle the rest of escape symbols
            .replace("\\", "\\\\")
            .replace("\"", "\\\"")
        )
    }
}
impl ToLiteral for String {
    fn to_literal(&self) -> Literal {
        (self as &str).to_literal()
    }
}

impl ToLiteral for Regex {
    fn to_literal(&self) -> Literal {
        let regex = self.as_str();

        // regexes that'd require rush-specific escaping are not supported
        if regex.contains("/") || regex.contains("\\") {
            panic!("ToLiteral for regexes containing (back)slashes is NYI");
        }

        String::from(regex)
    }
}

impl<T: ToLiteral> ToLiteral for [T] {
    fn to_literal(&self) -> Literal {
        format!("[{}]", self.iter()
            .map(T::to_literal).collect::<Vec<_>>().join(","))
    }
}

impl<K, V> ToLiteral for HashMap<K, V>
    where K: Hash + Eq + ToLiteral, V: ToLiteral
{
    fn to_literal(&self) -> Literal {
        format!("{{{}}}", self.iter()
            .map(|(ref k, ref v)| format!("{}:{}", k.to_literal(), v.to_literal()))
            .collect::<Vec<_>>().join(","))
    }
}
