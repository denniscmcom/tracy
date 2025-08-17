use crate::io::StructInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_quote;

pub fn impl_random_macro(input: StructInput) -> TokenStream {
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
        .fields
        .iter()
        .map(|f| {
            (
                quote! {#f: rng.random()},
                quote! {#f: rng.random_range(r.clone())},
            )
        })
        .unzip();

    let struct_ident = input.ident;
    let struct_ty = quote! {#struct_ident #ty_generics};
    let ty = if generics.params.is_empty() {
        quote! {f64}
    } else {
        quote! {#ty_generics}
    };

    let trait_ty = quote! {Random<#ty>};

    let random_impl = quote! {
        impl #impl_generics #trait_ty for #struct_ty #where_clause {

            fn random() -> Self {
                use rand::Rng;
                let mut rng = rand::rng();
                Self {
                    #(#fields),*
                }
            }

            fn random_range(r: std::ops::Range<#ty>) -> Self {
                use rand::Rng;
                let mut rng = rand::rng();
                Self {
                    #(#range_fields),*
                }
            }

        }
    };

    quote! { #random_impl }.into()
}
