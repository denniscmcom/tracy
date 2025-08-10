use crate::io::Input;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_quote;

pub fn impl_random_macro(input: Input) -> TokenStream {
    let mut generics = input.ast.generics.clone();
    let mut where_clause = generics.make_where_clause().clone();
    let predicates = &mut where_clause.predicates;

    for struct_generic_param in generics.params.iter() {
        if let syn::GenericParam::Type(ty) = struct_generic_param {
            predicates.push(parse_quote! {#ty: num_traits::Num});

            let distr = quote! {rand::distr};
            predicates.push(parse_quote! {#ty: #distr::uniform::SampleUniform});
            predicates.push(parse_quote! {#ty: core::cmp::PartialOrd});
            predicates.push(parse_quote! {#ty: Copy});
            predicates.push(parse_quote! {
                #distr::StandardUniform: #distr::Distribution<#ty>
            });
        }
    }

    let (impl_generics, ty_generics, _) = generics.split_for_impl();
    let (fields, range_fields): (Vec<_>, Vec<_>) = input
        .fields_idents
        .iter()
        .map(|f| {
            (
                quote! {#f: rng.random()},
                quote! {#f: rng.random_range(r.clone())},
            )
        })
        .unzip();

    let struct_ident = input.ident;
    let struct_ty: syn::Type = parse_quote! {#struct_ident #ty_generics};
    let random_impl = quote! {
        impl #impl_generics Random #ty_generics for #struct_ty #where_clause {
            fn random() -> Self {
                let mut rng = rand::rng();
                Self {
                    #(#fields),*
                }
            }

            fn random_range(r: std::ops::Range #ty_generics) -> Self {
                let mut rng = rand::rng();
                Self {
                    #(#range_fields),*
                }
            }

        }
    };

    quote! { #random_impl }.into()
}
