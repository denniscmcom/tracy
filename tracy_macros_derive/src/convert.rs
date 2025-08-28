use proc_macro::TokenStream;
use quote::quote;
use syn::GenericParam;
use tracy_macros_helper::StructInput;

pub fn impl_convert_macro(input: StructInput) -> TokenStream {
    let gens = input.ast.generics.clone();
    let (impl_gens, ty_gens, where_clause) = gens.split_for_impl();
    let fields = input.fields;
    let array_len = fields.iter().len();
    let ident = input.ident;

    if gens.params.len() > 1 {
        panic!("multiple generic parameters are not supported");
    }

    let mut trait_ty = quote! {f64};

    if let Some(GenericParam::Type(ty)) = gens.params.first() {
        trait_ty = quote! {#ty};
    }

    let convert_impl = quote! {
        impl #impl_gens Convert<#trait_ty, #array_len> for #ident #ty_gens
        #where_clause {
            fn as_array(&self) -> [#trait_ty; #array_len] {
                [ #(self.#fields),* ]
            }
        }
    };

    convert_impl.into()
}
