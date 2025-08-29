use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use std::fmt::Debug;
use syn::{
    GenericParam, Ident, Token, Type, parenthesized,
    parse::{Parse, ParseStream},
    parse_quote,
    token::{self},
};
use tracy_macros_helper::StructInput;

pub fn impl_ops_macro(attr: OpsAttr, input: StructInput) -> TokenStream {
    let mut ts2 = TokenStream2::new();
    let ast = &input.ast;
    ts2.extend(quote! {#ast});

    for (mut op, op_attr) in attr.ops {
        ts2.extend(op.generate(op_attr, &input));
    }

    ts2.into()
}

pub fn impl_add_macro(attr: OpAttr, input: StructInput) -> TokenStream {
    let mut op = Op::new(OpTy::Add);
    let ast = &input.ast;
    let generated = op.generate(attr, &input);
    quote! {#ast #generated}.into()
}

pub fn impl_sub_macro(attr: OpAttr, input: StructInput) -> TokenStream {
    let mut op = Op::new(OpTy::Sub);
    let ast = &input.ast;
    let generated = op.generate(attr, &input);
    quote! {#ast #generated}.into()
}

pub fn impl_mul_macro(attr: OpAttr, input: StructInput) -> TokenStream {
    let mut op = Op::new(OpTy::Mul);
    let ast = &input.ast;
    let generated = op.generate(attr, &input);
    quote! {#ast #generated}.into()
}

pub fn impl_div_macro(attr: OpAttr, input: StructInput) -> TokenStream {
    let mut op = Op::new(OpTy::Div);
    let ast = &input.ast;
    let generated = op.generate(attr, &input);
    quote! {#ast #generated}.into()
}

pub struct OpsAttr {
    pub ops: Vec<(Op, OpAttr)>,
}

impl Parse for OpsAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut ops = Vec::new();

        while !input.is_empty() {
            let op: Op = match input.parse::<Ident>()?.to_string().as_str() {
                "add" => Op::new(OpTy::Add),
                "sub" => Op::new(OpTy::Sub),
                "mul" => Op::new(OpTy::Mul),
                "div" => Op::new(OpTy::Div),
                _ => panic!("invalid op"),
            };

            let op_attr = if input.peek(token::Paren) {
                let attrs;
                parenthesized!(attrs in input);
                attrs.parse::<OpAttr>()?
            } else {
                OpAttr::default()
            };

            ops.push((op, op_attr));
            input.parse::<Option<Token![,]>>()?;
        }

        Ok(Self { ops })
    }
}

#[derive(Default)]
pub struct OpAttr {
    pub lhs_ty: Option<Type>,
    pub rhs_ty: Option<Type>,
    pub out_ty: Option<Type>,
}

impl Parse for OpAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut lhs_ty: Option<Type> = None;
        let mut rhs_ty: Option<Type> = None;
        let mut out_ty: Option<Type> = None;

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<Token![=]>()?;
            let val: Type = input.parse()?;

            match key.to_string().as_str() {
                "lhs" => lhs_ty = Some(val),
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

        Ok(Self {
            lhs_ty,
            rhs_ty,
            out_ty,
        })
    }
}

impl Debug for OpAttr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpAttr")
            .field(
                "lhs_ty",
                &self
                    .lhs_ty
                    .as_ref()
                    .map(|t| t.to_token_stream().to_string())
                    .unwrap_or_else(|| "None".to_string()),
            )
            .field(
                "rhs_ty",
                &self
                    .rhs_ty
                    .as_ref()
                    .map(|t| t.to_token_stream().to_string())
                    .unwrap_or_else(|| "None".to_string()),
            )
            .field(
                "out_ty",
                &self
                    .out_ty
                    .as_ref()
                    .map(|t| t.to_token_stream().to_string())
                    .unwrap_or_else(|| "None".to_string()),
            )
            .finish()
    }
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

    // FIXME:
    // This is valid:
    // struct Test<T> where T: Clone + Copy
    // This is not:
    // struct Test<T: Clone + Copy>

    pub fn generate(
        &mut self,
        attr: OpAttr,
        input: &StructInput,
    ) -> TokenStream2 {
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
        let ident = &input.ident;
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

impl Debug for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Op")
            .field("op", &self.op.to_token_stream())
            .field("op_trait", &self.op_trait.to_token_stream())
            .field("op_fn", &self.op_fn.to_token_stream())
            .field("op_assign", &self.op_assign.to_token_stream())
            .field("op_assign_trait", &self.op_assign_trait)
            .field("op_assign_fn", &self.op_assign_fn)
            .finish()
    }
}
