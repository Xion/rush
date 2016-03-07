//! Value type.

use std::collections::HashMap;
use std::convert::From;
use std::fmt;
use std::str::FromStr;

use rustc_serialize::json::{Json, ToJson};

use super::function::Function;


// Representations of various possible types of Value.
pub type SymbolRepr = String;
pub type BooleanRepr = bool;
pub type IntegerRepr = i64;
pub type FloatRepr = f64;
pub type StringRepr = String;
pub type ArrayRepr = Vec<Value>;
pub type ObjectRepr = HashMap<String, Value>;
pub type FunctionRepr = Function;


/// Typed value that's operated upon.
#[derive(Clone,PartialEq)]
pub enum Value {
    /// No value at all.
    Empty,

    /// Symbol is a string that can be interpreted as a variable name.
    ///
    /// `Symbol("x")` shall evaluate to the value of variable `x` if one is in scope.
    /// Otherwise, it should be equivalent to String("x").
    Symbol(SymbolRepr),

    // Various data types.
    Boolean(BooleanRepr),
    Integer(IntegerRepr),
    Float(FloatRepr),
    String(StringRepr),
    Array(ArrayRepr),
    Object(ObjectRepr),
    Function(FunctionRepr),
}


impl Value {
    /// Return the type of this value as string.
    /// These names are user-facing, e.g. they can occur inside error messages.
    pub fn typename(&self) -> &'static str {
        match *self {
            Value::Empty => "empty",
            Value::Symbol(..) => "symbol",
            Value::Boolean(..) => "bool",
            Value::Integer(..) => "int",
            Value::Float(..) => "float",
            Value::String(..) => "str",
            Value::Array(..) => "array",
            Value::Object(..) => "object",
            Value::Function(..) => "function",
        }
    }

    #[inline(always)]
    pub fn is_string(&self) -> bool {
        match *self { Value::String(..) => true, _ => false, }
    }
    #[inline(always)]
    pub fn is_int(&self) -> bool {
        match *self { Value::Integer(..) => true, _ => false, }
    }
    #[inline(always)]
    pub fn is_float(&self) -> bool {
        match *self { Value::Float(..) => true, _ => false, }
    }
    #[inline(always)]
    pub fn is_bool(&self) -> bool {
        match *self { Value::Boolean(..) => true, _ => false, }
    }
    #[inline(always)]
    pub fn is_array(&self) -> bool {
        match *self { Value::Array(..) => true, _ => false, }
    }
    #[inline(always)]
    pub fn is_object(&self) -> bool {
        match *self { Value::Object(..) => true, _ => false, }
    }
    #[inline(always)]
    pub fn is_function(&self) -> bool {
        match *self { Value::Function(..) => true, _ => false, }
    }

    #[inline]
    pub fn unwrap_string(self) -> StringRepr {
        match self {
            Value::String(s) => s,
            _ => { panic!("unwrap_string() on {} value", self.typename()) },
        }
    }
    #[inline]
    pub fn unwrap_int(self) -> IntegerRepr {
        match self {
            Value::Integer(i) => i,
            _ => { panic!("unwrap_int() on {} value", self.typename()) },
        }
    }
    #[inline]
    pub fn unwrap_float(self) -> FloatRepr {
        match self {
            Value::Float(f) => f,
            _ => { panic!("unwrap_float() on {} value", self.typename()) },
        }
    }
    #[inline]
    pub fn unwrap_bool(self) -> BooleanRepr {
        match self {
            Value::Boolean(b) => b,
            _ => { panic!("unwrap_bool() on {} value", self.typename()) },
        }
    }
    #[inline]
    pub fn unwrap_array(self) -> ArrayRepr {
        match self {
            Value::Array(a) => a,
            _ => { panic!("unwrap_array() on {} value", self.typename()) },
        }
    }
    #[inline]
    pub fn unwrap_object(self) -> ObjectRepr {
        match self {
            Value::Object(o) => o,
            _ => { panic!("unwrap_object() on {} value", self.typename()) },
        }
    }
    #[inline]
    pub fn unwrap_function(self) -> FunctionRepr {
        match self {
            Value::Function(f) => f,
            _ => { panic!("unwrap_function() on {} value", self.typename()) },
        }
    }
}


// Conversions from Rust types

/// Macro to create a straighforward From<FooRepr> -> Value::Foo implementation.
macro_rules! value_from (
    ($input:ty => $output:ident) => {
        impl From<$input> for Value {
            #[inline(always)]
            fn from(input: $input) -> Self {
                Value::$output(input)
            }
        }
    }
);

// Note how string input is deliberately omitted, for it is ambiguous.
// (It could result in either Value::String or Value::Symbol).
value_from!(IntegerRepr => Integer);
value_from!(FloatRepr => Float);
value_from!(BooleanRepr => Boolean);
value_from!(ArrayRepr => Array);
value_from!(ObjectRepr => Object);
value_from!(FunctionRepr => Function);


// TODO(xion): given the numerous ways we can & want to interpret the input,
// it makes less and less sense to have this as default;
// consider removing this impl
impl FromStr for Value {
    type Err = ();

    /// Create a Value from string, reinterpreting input as number
    /// if we find out it's in numeric form.
    fn from_str(s: &str) -> Result<Value, Self::Err> {
        if let Ok(int) = s.parse::<IntegerRepr>() {
            return Ok(Value::Integer(int));
        }
        if let Ok(float) = s.parse::<FloatRepr>() {
            return Ok(Value::Float(float));
        }
        if let Ok(boolean) = s.parse::<BooleanRepr>() {
            return Ok(Value::Boolean(boolean));
        }
        Ok(Value::String(s.to_owned()))
    }
}


// Debug & regular output

impl fmt::Debug for Value {
    /// Format a Value for debugging purposes.
    /// This representation is not meant for consumption by end users.
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Empty => write!(fmt, "{}", "<empty>"),
            Value::Symbol(ref t) => write!(fmt, ":{}", t),
            Value::Boolean(ref b) => write!(fmt, "{}", b.to_string()),
            Value::Integer(ref i) => write!(fmt, "{}i", i),
            Value::Float(ref f) => write!(fmt, "{}f", f),
            Value::String(ref s) => write!(fmt, "\"{}\"", s),
            Value::Array(ref a) => {
                write!(fmt, "[{}]", a.iter()
                    .map(|v| format!("{:?}", v)).collect::<Vec<String>>()
                    .join(","))
            },
            Value::Object(ref o) => {
                write!(fmt, "{{{}}}", o.iter()
                    .map(|(k, v)| format!("\"{}\": {:?}", k, v))
                    .collect::<Vec<String>>().join(","))
            },
            Value::Function(ref f) => write!(fmt, "{:?}", f),
        }
    }
}

// TODO(xion): Display doesn't really allow for propagating those errors
// (e.g. write!() never returns an I/O error on formatting error),
// so we may need a dedicated trait instead
impl fmt::Display for Value {
    /// Format a Value for outputing it as a result of the computation.
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // TODO(xion): make Empty a formatting error
            Value::Empty => write!(fmt, "{}", "<empty>"),
            Value::Symbol(ref t) => write!(fmt, "{}", t),
            Value::Boolean(ref b) => write!(fmt, "{}", b),
            Value::Integer(ref i) => write!(fmt, "{}", i),
            Value::Float(ref f) => {
                // always include decimal point and zero, even if the float
                // is actually an integer
                let mut res = f.to_string();
                if !res.contains(".") {
                    res.push_str(".0");
                }
                write!(fmt, "{}", res)
            },
            Value::String(ref s) => write!(fmt, "{}", s),
            Value::Array(ref a) => {
                // for final display, an array is assummed to contain lines of output
                write!(fmt, "{}", a.iter()
                    .map(|v| format!("{}", v)).collect::<Vec<String>>()
                    .join("\n"))
            },
            Value::Object(..) => write!(fmt, "{}", self.to_json().to_string()),
            // TODO(xion): make Function a formatting error
            Value::Function(..) => write!(fmt, "{}", "<function>"),
        }
    }
}


// JSON conversions

impl From<Json> for Value {
    fn from(input: Json) -> Self {
        match input {
            Json::Null => Value::Empty,
            Json::Boolean(b) => Value::Boolean(b),
            Json::I64(i) => Value::Integer(i),
            Json::U64(u) => {
                // TODO(xion): implement optional parsing
                if u > (IntegerRepr::max_value() as u64) {
                    panic!("JSON integer too large: {}", u);
                }
                Value::Integer(u as IntegerRepr)
            },
            Json::F64(f) => Value::Float(f),
            Json::String(s) => Value::String(s),
            Json::Array(a) => Value::Array(
                a.into_iter().map(Value::from).collect()
            ),
            Json::Object(o) => Value::Object(
                o.into_iter().map(|(k, v)| (k, Value::from(v))).collect()
            ),
        }
    }
}

impl ToJson for Value {
    /// Format the value as JSON.
    /// This is used for printing Object values as final output.
    fn to_json(&self) -> Json {
        match *self {
            Value::Empty => Json::Null,
            Value::Symbol(ref t) => Json::String(t.clone()),
            Value::Boolean(b) => Json::Boolean(b),
            Value::Integer(i) => Json::I64(i),
            Value::Float(f) => Json::F64(f),
            Value::String(ref s) => Json::String(s.clone()),
            Value::Array(ref a) => Json::Array(
                a.iter().map(|v| v.to_json()).collect()
            ),
            Value::Object(ref o) => Json::Object(
                o.iter().map(|(k, v)| (k.clone(), v.to_json())).collect()
            ),
            Value::Function(..) => panic!("function cannot be serialized as JSON"),
        }
    }
}
