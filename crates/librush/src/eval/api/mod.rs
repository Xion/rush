//! Module with built-in API that's available to the expressions.
//! This is basically the standard library of the language.

// NOTE: All actual API functions should be defined in submodules.

pub mod base;
pub mod conv;
pub mod math;
pub mod random;
pub mod strings;


use std::f64;

use eval::{Context, Value};
use eval::value::FloatRepr;


impl<'c> Context<'c> {
    /// Initialize symbols for the built-in functions and constants.
    /// This should be done only for the root Context (the one w/o a parent).
    pub fn init_builtins(&mut self) {
        assert!(self.is_root(), "Only root Context can have builtins!");
        self.init_functions();
        self.init_constants();
    }

    fn init_functions(&mut self) {
        //
        // Keep the list sorted alphabetically by function names.
        //
        self.define_unary(          "abs",      math::abs           );
        self.define_binary(         "after",    strings::after      );
        self.define_unary(          "all",      base::all           );
        self.define_unary(          "any",      base::any           );
        self.define_binary(         "before",   strings::before     );
        self.define_unary(          "bin",      math::bin           );
        self.define_unary(          "bool",     conv::bool          );
        self.define_unary(          "ceil",     math::ceil          );
        self.define_unary(          "char",     strings::chr        );
        self.define_unary(          "chr",      strings::chr        );
        self.define_unary(          "compact",  base::compact       );
        self.define_unary(          "csv",      conv::csv           );
        self.define_unary(          "exp",      math::exp           );
        self.define_binary_ctx(     "filter",   base::filter        );
        self.define_unary(          "float",    conv::float         );
        self.define_unary(          "floor",    math::floor         );
        self.define_ternary_ctx(    "fold",     base::reduce        );
        self.define_ternary_ctx(    "foldl",    base::reduce        );
        self.define_binary(         "format",   strings::format_    );
        self.define_ternary_ctx(    "gsub",     strings::sub        );
        self.define_unary(          "hex",      math::hex           );
        self.define_unary(          "id",       base::identity      );
        self.define_binary(         "index",    base::index         );
        self.define_unary(          "int",      conv::int           );
        self.define_binary(         "join",     strings::join       );
        self.define_unary(          "json",     conv::json          );
        self.define_unary(          "keys",     base::keys          );
        self.define_unary(          "latin1",   strings::latin1     );
        self.define_unary(          "len",      base::len           );
        self.define_unary(          "lines",    strings::lines      );
        self.define_unary(          "ln",       math::ln            );
        self.define_binary_ctx(     "map",      base::map           );
        self.define_unary_ctx(      "max",      base::max           );
        self.define_unary_ctx(      "min",      base::min           );
        self.define_unary(          "oct",      math::oct           );
        self.define_binary(         "omit",     base::omit          );
        self.define_unary(          "ord",      strings::ord        );
        self.define_binary(         "pick",     base::pick          );
        self.define_nullary(        "rand",     random::rand_       );
        self.define_unary(          "re",       conv::regex         );
        self.define_ternary_ctx(    "reduce",   base::reduce        );
        self.define_unary(          "regex",    conv::regex         );
        self.define_unary(          "regexp",   conv::regex         );
        self.define_binary_ctx(     "reject",   base::reject        );
        self.define_unary(          "rev",      base::rev           );
        self.define_unary(          "rot13",    strings::rot13      );
        self.define_unary(          "round",    math::round         );
        self.define_ternary(        "rsub1",    strings::rsub1      );
        self.define_binary(         "sample",   random::sample      );
        self.define_unary(          "sgn",      math::sgn           );
        self.define_unary(          "shuffle",  random::shuffle     );
        self.define_unary(          "sort",     base::sort          );
        self.define_binary_ctx(     "sortby",   base::sort_by       );
        self.define_binary(         "split",    strings::split      );
        self.define_unary(          "sqrt",     math::sqrt          );
        self.define_unary(          "str",      conv::str_          );
        self.define_ternary_ctx(    "sub",      strings::sub        );
        self.define_ternary_ctx(    "sub1",     strings::sub1       );
        self.define_unary_ctx(      "sum",      base::sum           );
        self.define_unary(          "trim",     strings::trim       );
        self.define_unary(          "trunc",    math::trunc         );
        self.define_unary(          "values",   base::values        );
        self.define_unary(          "words",    strings::words      );
    }

    fn init_constants(&mut self) {
        //
        // Keep the list sorted alphabetically by constant names (ignore case).
        //
        self.set(   "pi",       Value::Float(f64::consts::PI as FloatRepr)  );
    }
}


#[cfg(test)]
mod tests {
    use eval::Context;

    #[test]
    fn no_bool_constants() {
        let ctx = Context::new();
        for constant in &["true", "false"] {
            check_constant(&ctx, *constant);
        }
    }

    #[test]
    fn no_float_constants() {
        let ctx = Context::new();
        for constant in &["NaN", "Inf"] {
            check_constant(&ctx, *constant)
        }
    }

    #[test]
    fn no_nil() {
        let ctx = Context::new();
        check_constant(&ctx, "nil");
    }

    fn check_constant(ctx: &Context, name: &'static str) {
        assert!(!ctx.is_defined(name),
                "`{}` is handled by parser and doesn't need to be in Context", name);
    }
}
