mod math;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn add(attr: TokenStream, input: TokenStream) -> TokenStream {
    let (attr, ast) = extract_inputs(attr, input);
    math::impl_add_macro(attr, ast)
}

#[proc_macro_attribute]
pub fn sub(attr: TokenStream, input: TokenStream) -> TokenStream {
    let (attr, ast) = extract_inputs(attr, input);
    math::impl_sub_macro(attr, ast)
}

#[proc_macro_attribute]
pub fn mul(attr: TokenStream, input: TokenStream) -> TokenStream {
    let (attr, ast) = extract_inputs(attr, input);
    math::impl_mul_macro(attr, ast)
}

#[proc_macro_attribute]
pub fn div(attr: TokenStream, input: TokenStream) -> TokenStream {
    let (attr, ast) = extract_inputs(attr, input);
    math::impl_div_macro(attr, ast)
}

fn extract_inputs(
    attr: TokenStream,
    input: TokenStream,
) -> (Option<syn::Type>, syn::DeriveInput) {
    (
        syn::parse::<syn::Type>(attr).ok(),
        syn::parse(input).unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(false, true);
    }
}
