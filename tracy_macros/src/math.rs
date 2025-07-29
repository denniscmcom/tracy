use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{DeriveInput, Type, parse_quote};

pub fn impl_add_macro(attr: Option<Type>, ast: DeriveInput) -> TokenStream {
    let math_op = MathOp::new(MathOpTy::Add, &ast, attr);
    math_op.generate(ast).into()
}

pub fn impl_sub_macro(attr: Option<Type>, ast: DeriveInput) -> TokenStream {
    let math_op = MathOp::new(MathOpTy::Sub, &ast, attr);
    math_op.generate(ast).into()
}

pub fn impl_mul_macro(attr: Option<Type>, ast: DeriveInput) -> TokenStream {
    let math_op = MathOp::new(MathOpTy::Mul, &ast, attr);
    math_op.generate(ast).into()
}

pub fn impl_div_macro(attr: Option<Type>, ast: DeriveInput) -> TokenStream {
    let math_op = MathOp::new(MathOpTy::Div, &ast, attr);
    math_op.generate(ast).into()
}

enum MathOpTy {
    Add,
    Sub,
    Mul,
    Div,
}

struct MathOp {
    pub rhs_ty: syn::Type,
    pub is_rhs_ty_flat: bool,
    pub op: TokenStream2,
    pub op_trait: TokenStream2,
    pub op_fn: TokenStream2,
    pub op_assign: TokenStream2,
    pub op_assign_trait: TokenStream2,
    pub op_assign_fn: TokenStream2,
}

impl MathOp {
    fn new(
        op_ty: MathOpTy,
        ast: &DeriveInput,
        attr: Option<syn::Type>,
    ) -> Self {
        let rhs_ty = attr.unwrap_or_else(|| {
            let struct_ident = &ast.ident;
            let (_, ty_generics, _) = &ast.generics.split_for_impl();
            parse_quote!(#struct_ident #ty_generics)
        });

        let is_rhs_ty_flat = if let syn::Type::Path(rhs_ty_path) = &rhs_ty {
            let segment = rhs_ty_path.path.segments.first().unwrap();
            segment.arguments.is_empty()
        } else {
            false
        };

        let (op, op_trait, op_fn, op_assign, op_assign_trait, op_assign_fn) =
            match op_ty {
                MathOpTy::Add => (
                    quote! {+},
                    quote! {std::ops::Add},
                    quote! {add},
                    quote! {+=},
                    quote! {std::ops::AddAssign},
                    quote! {add_assign},
                ),
                MathOpTy::Sub => (
                    quote! {-},
                    quote! {std::ops::Sub},
                    quote! {sub},
                    quote! {-=},
                    quote! {std::ops::SubAssign},
                    quote! {sub_assign},
                ),
                MathOpTy::Mul => (
                    quote! {*},
                    quote! {std::ops::Mul},
                    quote! {mul},
                    quote! {*=},
                    quote! {std::ops::MulAssign},
                    quote! {mul_assign},
                ),
                MathOpTy::Div => (
                    quote! {/},
                    quote! {std::ops::Div},
                    quote! {div},
                    quote! {/=},
                    quote! {std::ops::DivAssign},
                    quote! {div_assign},
                ),
            };

        Self {
            rhs_ty,
            is_rhs_ty_flat,
            op,
            op_trait,
            op_fn,
            op_assign,
            op_assign_trait,
            op_assign_fn,
        }
    }

    fn generate(&self, ast: DeriveInput) -> TokenStream2 {
        let rhs_ty = &self.rhs_ty;
        let op = &self.op;
        let op_trait = &self.op_trait;
        let op_fn = &self.op_fn;
        let op_assign = &self.op_assign;
        let op_assign_trait = &self.op_assign_trait;
        let op_assign_fn = &self.op_assign_fn;

        let struct_fields = match &ast.data {
            syn::Data::Struct(data_struct) => &data_struct.fields,
            _ => panic!("#[MathOp] only supports structs"),
        };

        let struct_field_idents: Vec<_> = match struct_fields {
            syn::Fields::Named(named_fields) => named_fields
                .named
                .iter()
                .map(|f| f.ident.clone().unwrap())
                .collect(),
            _ => panic!("#[MathOp] only supports structs with named fields"),
        };

        let mut generics = ast.generics.clone();
        let mut where_clause = generics.make_where_clause().clone();
        let predicates = &mut where_clause.predicates;

        // TODO: Modify AST to derive automatically from Clone and Copy.

        for struct_generic_param in generics.params.iter() {
            if let syn::GenericParam::Type(ty) = struct_generic_param {
                // FIXME: Remove op_assign_trait from op_trait and viceverse.
                predicates.push(parse_quote! {
                    #ty: Copy + Clone + #op_trait<Output = #ty> +
                    #op_assign_trait
                });
            }
        }

        // TODO: Split op and op_assign code generation into two functions.
        let (impl_generics, ty_generics, _) = generics.split_for_impl();

        let (op_fields, op_assign_fields): (Vec<_>, Vec<_>) =
            struct_field_idents
                .iter()
                .map(|f| {
                    if self.is_rhs_ty_flat {
                        (
                            quote! {#f: self.#f #op rhs},
                            quote! {self.#f #op_assign rhs},
                        )
                    } else {
                        (
                            quote! {#f: self.#f #op rhs.#f},
                            quote! {self.#f #op_assign rhs.#f},
                        )
                    }
                })
                .unzip();

        let struct_ident = &ast.ident;
        let struct_ty: syn::Type = parse_quote! {#struct_ident #ty_generics};

        let op_impl = quote! {
            impl #impl_generics #op_trait<#rhs_ty> for #struct_ty
            #where_clause {
                type Output = #struct_ty;

                fn #op_fn(self, rhs: #rhs_ty) -> #struct_ty {
                    Self {
                       #(#op_fields),*
                    }
                }
            }
        };

        let op_assign_impl = quote! {
            impl #impl_generics #op_assign_trait<#rhs_ty> for #struct_ty
            #where_clause {
                fn #op_assign_fn(&mut self, rhs: #rhs_ty) {
                    #(#op_assign_fields);*
                }
            }
        };
        let generated = quote! {#ast #op_impl #op_assign_impl};

        generated.into()
    }
}
