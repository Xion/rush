//! Operator symbols.


named!(pub binary_op( &[u8] ) -> String, alt_complete!(
    functional_op | logical_op | comparison_op |
    power_op | multiplicative_op |  // power_op needs to be before multiplicative_op
    additive_op
));

named!(pub unary_op( &[u8] ) -> String, string!(multispaced!(
    char_of!("+-!")
)));


// Binary operators

named!(pub assignment_op( &[u8] ) -> String, string!(multispaced!(
    tag!("=")
)));
named!(pub functional_op( &[u8] ) -> String, string!(multispaced!(
    char_of!("&$")
)));
named!(pub logical_op( &[u8] ) -> String, string!(multispaced!(alt_complete!(
    tag!("&&") | tag!("||")
))));
named!(pub comparison_op( &[u8] ) -> String, string!(multispaced!(alt_complete!(
    tag!("<=") | tag!(">=") | tag!("==") | tag!("!=") | char_of!("<>@")
))));
named!(pub additive_op( &[u8] ) -> String, string!(multispaced!(
    char_of!("+-")
)));
named!(pub multiplicative_op( &[u8] ) -> String, string!(multispaced!(
    char_of!("*/%")
)));
named!(pub power_op( &[u8] ) -> String, string!(multispaced!(
    tag!("**")
)));
