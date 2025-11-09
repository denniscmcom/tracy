use proc_macro::TokenStream;
use quote::quote;
use syn::parse_quote;
use tracy_macros_helper::StructArrStore;

pub fn impl_neg_macro(mut input: StructArrStore) -> TokenStream {
    let mut where_clause = input.gens.make_where_clause().clone();
    let (impl_gens, ty_gens, _) = input.gens.split_for_impl();
    let name = &input.name;

    if input.has_generic_ty() {
        let ty_ident = &input.get_ty_param_ident();
        where_clause.predicates.push(parse_quote! {
            #ty_ident: std::ops::Neg<Output = #ty_ident> + Copy
        });
    }

    let output = if input.is_named() {
        let field_ident = input.field.ident.unwrap();
        quote! {
            Self {
                #field_ident: ArrStore(std::array::from_fn(|i| -self.arr[i])),
            }
        }
    } else {
        quote! {#name(-self.0)}
    };

    let expanded = quote! {
        impl #impl_gens std::ops::Neg for #name #ty_gens #where_clause {
            type Output = #name #ty_gens;
            fn neg(self) -> Self::Output {
                #output
            }
        }
    };

    TokenStream::from(expanded)
}
