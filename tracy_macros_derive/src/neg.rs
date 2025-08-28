use proc_macro::TokenStream;
use quote::quote;
use tracy_macros_helper::StructInput;

pub fn impl_neg_macro(input: StructInput) -> TokenStream {
    let gens = input.ast.generics.clone();
    let (impl_gens, ty_gens, where_clause) = gens.split_for_impl();
    let fields = input.fields.iter().map(|f| {
        quote! {#f: -self.#f}
    });

    let ident = input.ident;

    let neg_impl = quote! {
        impl #impl_gens std::ops::Neg for #ident #ty_gens #where_clause {
            type Output = #ident #ty_gens;

            fn neg(self) -> Self::Output {
                Self::Output {
                    #(#fields),*
                }
            }
        }
    };

    neg_impl.into()
}
