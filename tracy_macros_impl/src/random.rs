use crate::io::StructInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::{GenericParam, parse_quote};

pub fn impl_random_macro(input: StructInput) -> TokenStream {
    let mut gens = input.ast.generics.clone();
    let mut where_clause = gens.make_where_clause().clone();
    let pred = &mut where_clause.predicates;

    if gens.params.len() > 1 {
        panic!("multiple generic parameters are not supported");
    }

    let mut trait_ty = quote! {f64};

    if let Some(GenericParam::Type(ty)) = gens.params.first() {
        trait_ty = quote! {#ty};

        let distr = quote! {rand::distr};
        let std_uniform = quote! {#distr::StandardUniform};
        pred.push(parse_quote! {#ty: #distr::uniform::SampleUniform});
        pred.push(parse_quote! {#ty: core::cmp::PartialOrd});
        pred.push(parse_quote! {#ty: Copy});
        pred.push(parse_quote! {#std_uniform: #distr::Distribution<#ty>});
    }

    let (impl_gens, ty_gens, _) = gens.split_for_impl();
    let (fields, range_fields): (Vec<_>, Vec<_>) = input
        .fields
        .iter()
        .map(|f| {
            (
                quote! {#f: rng.random()},
                quote! {#f: rng.random_range(r.clone())},
            )
        })
        .unzip();

    let ident = input.ident;

    let random_impl = quote! {
        impl #impl_gens Random<#trait_ty> for #ident #ty_gens #where_clause {

            fn random() -> Self {
                use rand::Rng;
                let mut rng = rand::rng();
                Self {
                    #(#fields),*
                }
            }

            fn random_range(r: std::ops::Range<#trait_ty>) -> Self {
                use rand::Rng;
                let mut rng = rand::rng();
                Self {
                    #(#range_fields),*
                }
            }

        }
    };

    random_impl.into()
}
