use crate::attr::Attr;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::{GenericParam, Type, parse_quote};
use tracy_macros_helper::StructInput;

pub fn impl_add_macro(attr: Attr, input: StructInput) -> TokenStream {
    let mut math_op = Op::new(OpTy::Add);
    math_op.generate(attr, input).into()
}

pub fn impl_sub_macro(attr: Attr, input: StructInput) -> TokenStream {
    let mut math_op = Op::new(OpTy::Sub);
    math_op.generate(attr, input).into()
}

pub fn impl_mul_macro(attr: Attr, input: StructInput) -> TokenStream {
    let mut math_op = Op::new(OpTy::Mul);
    math_op.generate(attr, input).into()
}

pub fn impl_div_macro(attr: Attr, input: StructInput) -> TokenStream {
    let mut math_op = Op::new(OpTy::Div);
    math_op.generate(attr, input).into()
}

pub enum OpTy {
    Add,
    Sub,
    Mul,
    Div,
}

pub struct Op {
    op: TokenStream2,
    op_trait: TokenStream2,
    op_fn: TokenStream2,
    op_assign: TokenStream2,
    op_assign_trait: TokenStream2,
    op_assign_fn: TokenStream2,
    scalars: Vec<String>,
}

impl Op {
    pub fn new(op_ty: OpTy) -> Self {
        let scalars = vec![
            "f32", "f64", "u8", "u16", "u32", "u64", "usize", "i8", "i16",
            "i32", "i64", "isize",
        ]
        .into_iter()
        .map(|s| String::from(s))
        .collect();

        match op_ty {
            OpTy::Add => {
                return Self {
                    op: quote! {+},
                    op_trait: quote! {std::ops::Add},
                    op_fn: quote! {add},
                    op_assign: quote! {+=},
                    op_assign_trait: quote! {std::ops::AddAssign},
                    op_assign_fn: quote! {add_assign},
                    scalars,
                };
            }

            OpTy::Sub => {
                return Self {
                    op: quote! {-},
                    op_trait: quote! {std::ops::Sub},
                    op_fn: quote! {sub},
                    op_assign: quote! {-=},
                    op_assign_trait: quote! {std::ops::SubAssign},
                    op_assign_fn: quote! {sub_assign},
                    scalars,
                };
            }
            OpTy::Mul => {
                return Self {
                    op: quote! {*},
                    op_trait: quote! {std::ops::Mul},
                    op_fn: quote! {mul},
                    op_assign: quote! {*=},
                    op_assign_trait: quote! {std::ops::MulAssign},
                    op_assign_fn: quote! {mul_assign},
                    scalars,
                };
            }
            OpTy::Div => {
                return Self {
                    op: quote! {/},
                    op_trait: quote! {std::ops::Div},
                    op_fn: quote! {div},
                    op_assign: quote! {/=},
                    op_assign_trait: quote! {std::ops::DivAssign},
                    op_assign_fn: quote! {div_assign},
                    scalars,
                };
            }
        };
    }

    // TODO: Implement wrapper types to output a primitive type.
    // Example:
    // 10.0 degress / 5.0 degrees = 2.0 (unitless)
    // #[div(out = f64)]
    // struct Degrees(f64)
    pub fn generate(&mut self, attr: Attr, input: StructInput) -> TokenStream2 {
        let mut gens = input.ast.generics.clone();
        let mut where_clause = gens.make_where_clause().clone();

        let op_trait = &self.op_trait;
        let op_assign_trait = &self.op_assign_trait;

        // TODO: Only support one generic parameter at the moment.
        for g in gens.params.iter() {
            if let GenericParam::Type(ty) = g {
                self.scalars.push(ty.ident.to_string());

                // FIXME: Remove op trait from op assign and viceversa.
                let pred = &mut where_clause.predicates;
                pred.push(parse_quote! {#ty: #op_trait<Output = #ty>});
                pred.push(parse_quote! {#ty: #op_assign_trait});
            }
        }

        let (impl_gens, ty_gens, _) = gens.split_for_impl();
        let ident = input.ident;
        let op_fn = &self.op_fn;

        let op = &self.op;
        let op_trait = &self.op_trait;
        let out_ty = attr
            .out_ty
            .clone()
            .unwrap_or(parse_quote! {#ident #ty_gens});

        let build_op = |rhs_ty: &Type| -> TokenStream2 {
            let fields = input.fields.iter().map(|f| {
                if self.is_scalar(&rhs_ty) {
                    return quote! {#f: self.#f #op rhs.clone()};
                }

                quote! {#f: self.#f #op rhs.#f}
            });

            quote! {
                impl #impl_gens #op_trait<#rhs_ty> for #ident #ty_gens
                #where_clause {
                    type Output = #out_ty;
                    fn #op_fn(self, rhs: #rhs_ty) -> Self::Output {
                        Self::Output {
                            #(#fields),*
                        }
                    }
                }
            }
        };

        // TODO: Integrate this closure in a single build_op.
        let build_op_lhs = |lhs_ty: &Type| -> TokenStream2 {
            let fields = input.fields.iter().map(|f| {
                if self.is_scalar(&lhs_ty) {
                    return quote! {#f: rhs.#f #op self.clone()};
                }

                quote! {#f: rhs.#f #op self.#f}
            });

            quote! {
                impl #impl_gens #op_trait<#ident #ty_gens> for #lhs_ty
                #where_clause {
                    type Output = #out_ty;
                    fn #op_fn(self, rhs: #ident #ty_gens) -> Self::Output {
                        Self::Output {
                            #(#fields),*
                        }
                    }
                }
            }
        };

        let build_op_assign = |rhs_ty: &Type| -> TokenStream2 {
            let op_assign = &self.op_assign;
            let op_assign_fn = &self.op_assign_fn;
            let op_assign_trait = &self.op_assign_trait;
            let fields = input.fields.iter().map(|f| {
                if self.is_scalar(&rhs_ty) {
                    return quote! {self.#f #op_assign rhs.clone()};
                }

                quote! {self.#f #op_assign rhs.#f}
            });

            quote! {
                impl #impl_gens #op_assign_trait<#rhs_ty> for #ident #ty_gens
                #where_clause {
                    fn #op_assign_fn(&mut self, rhs: #rhs_ty) {
                        #(#fields);*
                    }
                }
            }
        };

        let mut tokens = Vec::new();
        tokens.push(input.ast.to_token_stream());

        if let Some(lhs_ty) = attr.lhs_ty {
            tokens.push(build_op_lhs(&lhs_ty));

            if let Some(rhs_ty) = attr.rhs_ty {
                tokens.push(build_op(&rhs_ty));
            }
        } else {
            let rhs_ty = attr.rhs_ty.unwrap_or(parse_quote! {#ident #ty_gens});
            tokens.push(build_op(&rhs_ty));
            tokens.push(build_op_assign(&rhs_ty));
        }

        tokens.into_iter().collect()
    }

    fn is_scalar(&self, ty: &Type) -> bool {
        if let Type::Path(ty_path) = ty {
            return ty_path
                .path
                .segments
                .iter()
                .any(|s| self.scalars.contains(&s.ident.to_string()));
        }

        false
    }
}
