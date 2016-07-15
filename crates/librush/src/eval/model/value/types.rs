//! Value types and related logic.
//!
//! Note that "type" is not a first-class concept in the language.
//! The various Value types are just a set of predefined variants.

use std::collections::HashMap;

use regex::Regex;

use eval::model::Function;
use super::Value;


// Representations of various possible types of Value.
pub type SymbolRepr = String;
pub type BooleanRepr = bool;
pub type IntegerRepr = i64;
pub type FloatRepr = f64;
pub type StringRepr = String;
pub type RegexRepr = Regex;
pub type ArrayRepr = Vec<Value>;
pub type ObjectRepr = HashMap<String, Value>;
pub type FunctionRepr = Function;


/// Macro to implement methods on Value that deal with its various types.
/// For a type X, those methods are is_X(), unwrap_X(), and as_X().
macro_rules! impl_value_type {
    // Unfortunately, due to Rust's strict macro hygiene requirements
    // and the general uselessness of concat_idents!, all the method names have to be given
    // as macro arguments.
    // More details: https://github.com/rust-lang/rust/issues/12249
    ($variant:ident($t:ty) => ($is:ident, $unwrap:ident, $as_:ident)) => (
        impl Value {
            /// Check whether Value is of type $t.
            #[inline(always)]
            pub fn $is(&self) -> bool {
                match *self { Value::$variant(..) => true, _ => false, }
            }

            /// Consumes the Value, returning the underlying $t.
            /// Panics if the Value is not a $t.
            #[inline]
            pub fn $unwrap(self) -> $t {
                match self {
                    Value::$variant(x) => x,
                    _ => panic!(concat!(stringify!($unwrap), "() on {} value"), self.typename()),
                }
            }

            /// Returns a reference to the underlying $t.
            /// Panics if the Value is not a $t.
            #[inline]
            pub fn $as_(&self) -> &$t {
                match self {
                    &Value::$variant(ref x) => x,
                    _ => panic!(concat!(stringify!($as_), "() on {} value"), self.typename()),
                }
            }
        }
    );
}

impl_value_type!(Boolean(BooleanRepr)   => (is_bool,     unwrap_bool,     as_bool));
impl_value_type!(Integer(IntegerRepr)   => (is_integer,  unwrap_integer,  as_integer));
impl_value_type!(Integer(IntegerRepr)   => (is_int,      unwrap_int,      as_int));  // alias
impl_value_type!(Float(FloatRepr)       => (is_float,    unwrap_float,    as_float));
impl_value_type!(String(StringRepr)     => (is_string,   unwrap_string,   as_string));
impl_value_type!(String(StringRepr)     => (is_str,      unwrap_str,      as_str));  // alias
impl_value_type!(Regex(RegexRepr)       => (is_regex,    unwrap_regex,    as_regex));
impl_value_type!(Array(ArrayRepr)       => (is_array,    unwrap_array,    as_array));
impl_value_type!(Object(ObjectRepr)     => (is_object,   unwrap_object,   as_object));
impl_value_type!(Function(FunctionRepr) => (is_function, unwrap_function, as_function));


/// Additional methods that deal with more than one type at once.
impl Value {
    #[inline(always)]
    pub fn is_scalar(&self) -> bool {
        self.is_bool() || self.is_int() || self.is_float() || self.is_string()
    }

    #[inline(always)]
    pub fn is_number(&self) -> bool {
        self.is_int() || self.is_float()
    }
}
