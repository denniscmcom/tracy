use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Ident, parse_quote};
use tracy_macros_helper::StructArrStore;

pub fn impl_random_macro(mut input: StructArrStore) -> TokenStream {
    let mut where_clause = input.gens.make_where_clause().clone();
    let (impl_gens, ty_gens, _) = input.gens.split_for_impl();
    let name = &input.name;

    let ty_ident = input.get_ty_param_ident();

    if input.has_generic_ty() {
        where_clause.predicates.push(parse_quote! {
            #ty_ident: rand::distr::uniform::SampleUniform + Clone
        });

        where_clause.predicates.push(parse_quote! {
            rand::distr::StandardUniform: rand::distr::Distribution<
            #ty_ident
            >
        });
    }

    let output = if input.is_named() {
        let field_ident = input.field.ident.unwrap();
        (
            quote! {Self {
                #field_ident: ArrStore(
                    std::array::from_fn(|_| rng.random()),
                )
            }},
            quote! {Self {
                #field_ident: ArrStore(
                    std::array::from_fn(
                        |_| rng.random_range(r.clone())
                    )
                )
            }},
        )
    } else {
        (
            quote! {Self(rng.random())},
            quote! {Self(rng.random_range(r.clone()))},
        )
    };

    let expand_impl =
        |ty_param_ident: &Ident, output: (TokenStream2, TokenStream2)| {
            let (rand_output, rand_range_output) = output;

            quote! {
                impl #impl_gens Random<#ty_param_ident> for #name #ty_gens
                #where_clause {
                    fn random() -> Self {
                        use rand::Rng;
                        let mut rng = rand::rng();
                        #rand_output
                    }

                    fn random_range<R>(r: R) -> Self
                    where
                        R: Clone
                        + rand::distr::uniform::SampleRange<#ty_param_ident>
                    {
                        use rand::Rng;
                        let mut rng = rand::rng();
                        #rand_range_output
                    }
                }
            }
        };

    let expanded = expand_impl(&ty_ident, output);

    TokenStream::from(expanded.clone())
}
