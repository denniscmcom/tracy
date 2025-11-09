use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    GenericParam, Ident, Token, parse::Parse, punctuated::Punctuated,
    token::Comma,
};
use tracy_macros_helper::{StructArrStore, struct_arr_store::Form};

pub fn impl_fields_macro(
    attr: FieldsAttr,
    input: StructArrStore,
) -> TokenStream {
    let fields_idents = &mut attr.fields_idents.clone();

    if let Form::Generic(_, arr_store_len) | Form::Concrete(_, arr_store_len) =
        &input.form
    {
        if *arr_store_len != fields_idents.iter().count() {
            panic!("expected 'ArrStore' to match the number of fields idents");
        }
    }

    let mut gens = input.gens.clone();

    // Remove generic const parameter from impl.
    gens.params = gens
        .params
        .into_iter()
        .filter(|p| !matches!(p, GenericParam::Const(_)))
        .collect::<Punctuated<_, Comma>>();

    let name = &input.name;
    let ty_gen_ident = input.get_ty_param_ident();
    let (impl_gens, _, where_clause) = &gens.split_for_impl();

    let mut expanded = quote! {};

    while !fields_idents.is_empty() {
        let fields_idents_count = fields_idents.iter().count();
        let mut struct_ty = quote! {#name};

        if input.has_generic_ty() {
            struct_ty.extend(quote! {<#ty_gen_ident});

            if input.has_generic_const() {
                struct_ty.extend(quote! {, #fields_idents_count>});
            } else {
                struct_ty.extend(quote! {>});
            }
        } else if input.has_generic_const() {
            struct_ty.extend(quote! {<#fields_idents_count>});
        }

        let new_fn_args = quote! {#(#fields_idents: #ty_gen_ident),*};
        let new_fn_out = quote! {#(#fields_idents),*};

        let new_fn = quote! {
            pub fn new(#new_fn_args) -> Self {
                Self {
                    arr: arr_store::ArrStore([#new_fn_out]),
                }
            }
        };

        let mut accessors = quote! {};

        for (i, f) in fields_idents.iter().enumerate() {
            let f_mut = format_ident!("{}_mut", f);

            accessors.extend(quote! {
                pub fn #f(&self) -> #ty_gen_ident {
                    self.arr[#i]
                }

                pub fn #f_mut(&mut self) -> &mut #ty_gen_ident {
                    &mut self.arr[#i]
                }
            });
        }

        expanded.extend(quote! {
            impl #impl_gens #struct_ty #where_clause {
                #new_fn
                #accessors
            }
        });

        if input.has_generic_const() {
            fields_idents.pop();
            continue;
        }

        fields_idents.clear();
    }

    let ast = input.ast.clone();
    expanded.extend(quote! {#ast});

    TokenStream::from(expanded)
}

#[derive(Debug)]
pub struct FieldsAttr {
    pub fields_idents: Vec<Ident>,
}

impl Parse for FieldsAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut fields_idents = Vec::new();

        while !input.is_empty() {
            let field: Ident = input.parse()?;
            fields_idents.push(field);

            if let Ok(_) = input.parse::<Token![,]>() {
                continue;
            }

            break;
        }

        Ok(Self { fields_idents })
    }
}
