mod color;
mod convert;
mod neg;
mod random;

use proc_macro::TokenStream;
use syn::parse_macro_input;

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

#[proc_macro_derive(Convert)]
pub fn convert_derive(input: TokenStream) -> TokenStream {
    let parsed_input = parse_macro_input!(input);
    convert::impl_convert_macro(parsed_input)
}

#[proc_macro_derive(Color)]
pub fn color_derive(input: TokenStream) -> TokenStream {
    let parsed_input = parse_macro_input!(input);
    color::impl_color_macro(parsed_input)
}
