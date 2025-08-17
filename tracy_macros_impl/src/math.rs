use crate::io::StructInput;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    GenericParam, Ident, Token, Type,
    parse::{Parse, ParseStream},
    parse_quote,
};

pub fn impl_add_macro(attr: Attr, input: StructInput) -> TokenStream {
    let mut math_op = MathOp::new(MathOpTy::Add);
    math_op.generate(attr, input).into()
}

pub fn impl_sub_macro(attr: Attr, input: StructInput) -> TokenStream {
    let mut math_op = MathOp::new(MathOpTy::Sub);
    math_op.generate(attr, input).into()
}

pub fn impl_mul_macro(attr: Attr, input: StructInput) -> TokenStream {
    let mut math_op = MathOp::new(MathOpTy::Mul);
    math_op.generate(attr, input).into()
}

pub fn impl_div_macro(attr: Attr, input: StructInput) -> TokenStream {
    let mut math_op = MathOp::new(MathOpTy::Div);
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
                        format!("unknown attribute key `{}`", other),
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
    op: TokenStream2,
    op_trait: TokenStream2,
    op_fn: TokenStream2,
    op_assign: TokenStream2,
    op_assign_trait: TokenStream2,
    op_assign_fn: TokenStream2,
    scalar_tys: Vec<String>,
}

impl MathOp {
    fn new(op_ty: MathOpTy) -> Self {
        let scalar_tys = vec![
            "f32", "f64", "u8", "u16", "u32", "u64", "usize", "i8", "i16",
            "i32", "i64", "isize",
        ]
        .into_iter()
        .map(|s| String::from(s))
        .collect();

        match op_ty {
            MathOpTy::Add => {
                return Self {
                    op: quote! {+},
                    op_trait: quote! {std::ops::Add},
                    op_fn: quote! {add},
                    op_assign: quote! {+=},
                    op_assign_trait: quote! {std::ops::AddAssign},
                    op_assign_fn: quote! {add_assign},
                    scalar_tys,
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
                    scalar_tys,
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
                    scalar_tys,
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
                    scalar_tys,
                };
            }
        };
    }

    fn generate(&mut self, attr: Attr, input: StructInput) -> TokenStream2 {
        let op_trait = &self.op_trait;
        let op_assign_trait = &self.op_assign_trait;
        let op = &self.op;
        let op_assign = &self.op_assign;

        let mut generics = input.ast.generics.clone();
        let mut where_clause = generics.make_where_clause().clone();

        let (impl_generics, ty_generics, _) = generics.split_for_impl();
        let struct_ident = input.ident;
        let struct_ty: syn::Type = parse_quote! {#struct_ident #ty_generics};
        let (rhs_ty, out_ty) = attr.unwrap(&struct_ty);

        generics.params.iter().for_each(|g| {
            if let GenericParam::Type(ty) = g {
                let predicates = &mut where_clause.predicates;
                self.scalar_tys.push(ty.ident.to_string());
                predicates.push(parse_quote! {#ty: Clone});
                predicates.push(parse_quote! {#ty: Copy});
                predicates.push(parse_quote! {#ty: #op_trait<Output = #ty>});
                predicates.push(parse_quote! {#ty: #op_assign_trait});
            }
        });

        let (op_fields, op_assign_fields): (Vec<_>, Vec<_>) = input
            .fields
            .iter()
            .map(|f| {
                if self.is_scalar(&rhs_ty) {
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

    fn is_scalar(&self, ty: &Type) -> bool {
        if let Type::Path(ty_path) = ty {
            return ty_path
                .path
                .segments
                .iter()
                .any(|s| self.scalar_tys.contains(&s.ident.to_string()));
        }

        false
    }
}
