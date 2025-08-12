use crate::io::Input;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    Ident, Token, Type,
    parse::{Parse, ParseStream},
    parse_quote,
};

pub fn impl_add_macro(attr: Attr, input: Input) -> TokenStream {
    let math_op = MathOp::new(MathOpTy::Add);
    math_op.generate(attr, input).into()
}

pub fn impl_sub_macro(attr: Attr, input: Input) -> TokenStream {
    let math_op = MathOp::new(MathOpTy::Sub);
    math_op.generate(attr, input).into()
}

pub fn impl_mul_macro(attr: Attr, input: Input) -> TokenStream {
    let math_op = MathOp::new(MathOpTy::Mul);
    math_op.generate(attr, input).into()
}

pub fn impl_div_macro(attr: Attr, input: Input) -> TokenStream {
    let math_op = MathOp::new(MathOpTy::Div);
    math_op.generate(attr, input).into()
}

pub struct Attr {
    pub rhs_ty: Option<Type>,
    pub out_ty: Option<Type>,
}

impl Parse for Attr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut rhs_ty: Option<Type> = None;
        let mut out_ty: Option<Type> = None;

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<Token![=]>()?;
            let val: Type = input.parse()?;

            match key.to_string().as_str() {
                "rhs" => rhs_ty = Some(val),
                "out" => out_ty = Some(val),
                other => {
                    return Err(syn::Error::new(
                        key.span(),
                        format!("Unknown attribute key `{}`", other),
                    ));
                }
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(Self { rhs_ty, out_ty })
    }
}

impl Attr {
    fn unwrap(self, self_ty: &Type) -> (Type, Type) {
        (
            self.rhs_ty.unwrap_or_else(|| parse_quote! {#self_ty}),
            self.out_ty.unwrap_or_else(|| parse_quote! {#self_ty}),
        )
    }
}

enum MathOpTy {
    Add,
    Sub,
    Mul,
    Div,
}

struct MathOp {
    pub op: TokenStream2,
    pub op_trait: TokenStream2,
    pub op_fn: TokenStream2,
    pub op_assign: TokenStream2,
    pub op_assign_trait: TokenStream2,
    pub op_assign_fn: TokenStream2,
}

impl MathOp {
    fn new(op_ty: MathOpTy) -> Self {
        match op_ty {
            MathOpTy::Add => {
                return Self {
                    op: quote! {+},
                    op_trait: quote! {std::ops::Add},
                    op_fn: quote! {add},
                    op_assign: quote! {+=},
                    op_assign_trait: quote! {std::ops::AddAssign},
                    op_assign_fn: quote! {add_assign},
                };
            }

            MathOpTy::Sub => {
                return Self {
                    op: quote! {-},
                    op_trait: quote! {std::ops::Sub},
                    op_fn: quote! {sub},
                    op_assign: quote! {-=},
                    op_assign_trait: quote! {std::ops::SubAssign},
                    op_assign_fn: quote! {sub_assign},
                };
            }
            MathOpTy::Mul => {
                return Self {
                    op: quote! {*},
                    op_trait: quote! {std::ops::Mul},
                    op_fn: quote! {mul},
                    op_assign: quote! {*=},
                    op_assign_trait: quote! {std::ops::MulAssign},
                    op_assign_fn: quote! {mul_assign},
                };
            }
            MathOpTy::Div => {
                return Self {
                    op: quote! {/},
                    op_trait: quote! {std::ops::Div},
                    op_fn: quote! {div},
                    op_assign: quote! {/=},
                    op_assign_trait: quote! {std::ops::DivAssign},
                    op_assign_fn: quote! {div_assign},
                };
            }
        };
    }

    fn generate(&self, attr: Attr, input: Input) -> TokenStream2 {
        let op_trait = &self.op_trait;
        let op_assign_trait = &self.op_assign_trait;
        let op = &self.op;
        let op_assign = &self.op_assign;

        let mut generics = input.ast.generics.clone();
        let mut where_clause = generics.make_where_clause().clone();
        let predicates = &mut where_clause.predicates;

        let (impl_generics, ty_generics, _) = generics.split_for_impl();
        let struct_ident = input.ident;
        let struct_ty: syn::Type = parse_quote! {#struct_ident #ty_generics};
        let (rhs_ty, out_ty) = attr.unwrap(&struct_ty);

        let is_scalar_ty = |ty: &Type| {
            if let syn::Type::Path(ty_path) = ty {
                if let Some(segment) = ty_path.path.segments.first() {
                    let ident_str = segment.ident.to_string();
                    // TODO: Improve this (remove harcoded T).
                    if ident_str == "T" {
                        return true;
                    }

                    return ident_str == "f32"
                        || ident_str == "f64"
                        || ident_str == "u8"
                        || ident_str == "u16"
                        || ident_str == "u32"
                        || ident_str == "u64"
                        || ident_str == "usize"
                        || ident_str == "i8"
                        || ident_str == "i16"
                        || ident_str == "i32"
                        || ident_str == "i64"
                        || ident_str == "isize";
                }
            }

            false
        };

        let (op_fields, op_assign_fields): (Vec<_>, Vec<_>) = input
            .fields_idents
            .iter()
            .map(|f| {
                if is_scalar_ty(&rhs_ty) {
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

        // FIXME: Remove assign traits from op_impl and viceversa.
        for struct_generic_param in generics.params.iter() {
            if let syn::GenericParam::Type(ty) = struct_generic_param {
                predicates.push(parse_quote! {#ty: Clone});
                predicates.push(parse_quote! {#ty: Copy});
                predicates.push(parse_quote! {#ty: #op_trait<Output = #ty>});
                predicates.push(parse_quote! {#ty: #op_assign_trait});
            }
        }

        let op_fn = &self.op_fn;
        let out_ident = match &out_ty {
            Type::Path(out_ty_path) => {
                &out_ty_path.path.segments.last().unwrap().ident
            }
            _ => panic!("failed to extract ident from out_ty"),
        };

        let op_impl = quote! {
            impl #impl_generics #op_trait<#rhs_ty> for #struct_ty
            #where_clause {
                type Output = #out_ty;

                fn #op_fn(self, rhs: #rhs_ty) -> #out_ty {
                    #out_ident {
                       #(#op_fields),*
                    }
                }
            }
        };

        let op_assign_fn = &self.op_assign_fn;

        let op_assign_impl = quote! {
            impl #impl_generics #op_assign_trait<#rhs_ty> for #struct_ty
            #where_clause {
                fn #op_assign_fn(&mut self, rhs: #rhs_ty) {
                    #(#op_assign_fields);*
                }
            }
        };

        let ast = input.ast;
        let generated = quote! {#ast #op_impl #op_assign_impl};

        generated.into()
    }
}
