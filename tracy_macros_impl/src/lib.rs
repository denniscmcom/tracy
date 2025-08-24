mod io;
mod math;
mod neg;
mod random;

use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn add(attr: TokenStream, input: TokenStream) -> TokenStream {
    let parsed_attr = parse_macro_input!(attr);
    let parsed_input = parse_macro_input!(input);
    math::impl_add_macro(parsed_attr, parsed_input)
}

#[proc_macro_attribute]
pub fn sub(attr: TokenStream, input: TokenStream) -> TokenStream {
    let parsed_attr = parse_macro_input!(attr);
    let parsed_input = parse_macro_input!(input);
    math::impl_sub_macro(parsed_attr, parsed_input)
}

#[proc_macro_attribute]
pub fn mul(attr: TokenStream, input: TokenStream) -> TokenStream {
    let parsed_attr = parse_macro_input!(attr);
    let parsed_input = parse_macro_input!(input);
    math::impl_mul_macro(parsed_attr, parsed_input)
}

#[proc_macro_attribute]
pub fn div(attr: TokenStream, input: TokenStream) -> TokenStream {
    let parsed_attr = parse_macro_input!(attr);
    let parsed_input = parse_macro_input!(input);
    math::impl_div_macro(parsed_attr, parsed_input)
}

#[proc_macro_derive(Random)]
pub fn rand_derive(input: TokenStream) -> TokenStream {
    let parsed_input = parse_macro_input!(input);
    random::impl_random_macro(parsed_input)
}

#[proc_macro_derive(Neg)]
pub fn neg_derive(input: TokenStream) -> TokenStream {
    let parsed_input = parse_macro_input!(input);
    neg::impl_neg_macro(parsed_input)
}
