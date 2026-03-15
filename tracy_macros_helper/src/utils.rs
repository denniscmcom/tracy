use quote::{ToTokens, quote};
use syn::{Ident, parse_str};

pub fn get_ident<T>(t: T) -> syn::Result<Ident>
where
    T: ToTokens,
{
    parse_str(quote! {#t}.to_string().as_str())
}
