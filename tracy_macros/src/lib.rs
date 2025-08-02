mod math;

use proc_macro::TokenStream;
use syn::parse_macro_input;

// TODO: Add a global math macro to group multiple operations.

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

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         assert_eq!(false, true);
//     }
// }
