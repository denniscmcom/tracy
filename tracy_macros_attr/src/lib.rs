mod fields;
mod ops;

use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn fields(attr: TokenStream, input: TokenStream) -> TokenStream {
    let parse_attr = parse_macro_input!(attr);
    let parsed_input = parse_macro_input!(input);
    fields::impl_fields_macro(parse_attr, parsed_input)
}

#[proc_macro_attribute]
pub fn ops(attr: TokenStream, input: TokenStream) -> TokenStream {
    let parsed_attr = parse_macro_input!(attr);
    let parsed_input = parse_macro_input!(input);
    ops::impl_ops_macro(parsed_attr, parsed_input)
}

#[proc_macro_attribute]
pub fn add(attr: TokenStream, input: TokenStream) -> TokenStream {
    let parsed_attr = parse_macro_input!(attr);
    let parsed_input = parse_macro_input!(input);
    ops::impl_add_macro(parsed_attr, parsed_input)
}

#[proc_macro_attribute]
pub fn sub(attr: TokenStream, input: TokenStream) -> TokenStream {
    let parsed_attr = parse_macro_input!(attr);
    let parsed_input = parse_macro_input!(input);
    ops::impl_sub_macro(parsed_attr, parsed_input)
}

#[proc_macro_attribute]
pub fn mul(attr: TokenStream, input: TokenStream) -> TokenStream {
    let parsed_attr = parse_macro_input!(attr);
    let parsed_input = parse_macro_input!(input);
    ops::impl_mul_macro(parsed_attr, parsed_input)
}

#[proc_macro_attribute]
pub fn div(attr: TokenStream, input: TokenStream) -> TokenStream {
    let parsed_attr = parse_macro_input!(attr);
    let parsed_input = parse_macro_input!(input);
    ops::impl_div_macro(parsed_attr, parsed_input)
}
