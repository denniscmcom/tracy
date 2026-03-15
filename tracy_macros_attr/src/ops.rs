use std::fmt::Debug;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::{
    GenericParam, Ident, Token, Type, parenthesized,
    parse::{Parse, ParseStream},
    parse_quote, parse_str, parse2,
    punctuated::Punctuated,
    token::{self, Comma},
};
use tracy_macros_helper::StructArrStore;

pub fn impl_ops_macro(attr: OpsAttr, mut input: StructArrStore) -> TokenStream {
    let mut ts2 = TokenStream2::new();
    let ast = &input.ast;
    ts2.extend(quote! {#ast});

    for (mut op, op_attr) in attr.ops {
        ts2.extend(op.expand(op_attr, &mut input));
    }

    ts2.into()
}

/// Argument of the #[ops()] macro.
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

pub fn impl_add_macro(attr: OpAttr, mut input: StructArrStore) -> TokenStream {
    let mut op = Op::new(OpTy::Add);
    let op_expanded = op.expand(attr, &mut input);
    let ast = &input.ast;
    let expanded = quote! {#ast #op_expanded};
    TokenStream::from(expanded)
}

pub fn impl_sub_macro(attr: OpAttr, mut input: StructArrStore) -> TokenStream {
    let mut op = Op::new(OpTy::Sub);
    let op_expanded = op.expand(attr, &mut input);
    let ast = &input.ast;
    let expanded = quote! {#ast #op_expanded};
    TokenStream::from(expanded)
}

pub fn impl_mul_macro(attr: OpAttr, mut input: StructArrStore) -> TokenStream {
    let mut op = Op::new(OpTy::Mul);
    let op_expanded = op.expand(attr, &mut input);
    let ast = &input.ast;
    let expanded = quote! {#ast #op_expanded};
    TokenStream::from(expanded)
}

pub fn impl_div_macro(attr: OpAttr, mut input: StructArrStore) -> TokenStream {
    let mut op = Op::new(OpTy::Div);
    let op_expanded = op.expand(attr, &mut input);
    let ast = &input.ast;
    let expanded = quote! {#ast #op_expanded};
    TokenStream::from(expanded)
}

/// Argument of each arithmetic macro.
#[derive(Default)]
pub struct OpAttr {
    pub lhs: Option<TokenStream2>,
    pub rhs: Option<TokenStream2>,
    pub out: Option<TokenStream2>,
}

impl Parse for OpAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut lhs: Option<TokenStream2> = None;
        let mut rhs: Option<TokenStream2> = None;
        let mut out: Option<TokenStream2> = None;

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<Token![=]>()?;
            let val_ty: syn::Type = input.parse()?;
            let val = quote! { #val_ty };

            match key.to_string().as_str() {
                "lhs" => lhs = Some(val),
                "rhs" => rhs = Some(val),
                "out" => out = Some(val),
                other => {
                    return Err(syn::Error::new(
                        key.span(),
                        format!("unknown attribute key `{}`", other),
                    ));
                }
            }

            if let Ok(_) = input.parse::<Token![,]>() {
                continue;
            }

            break;
        }

        Ok(Self { lhs, rhs, out })
    }
}

impl Debug for OpAttr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpAttr")
            .field(
                "lhs_ty",
                &self
                    .lhs
                    .as_ref()
                    .map(|t| t.to_token_stream().to_string())
                    .unwrap_or_else(|| "None".to_string()),
            )
            .field(
                "rhs_ty",
                &self
                    .rhs
                    .as_ref()
                    .map(|t| t.to_token_stream().to_string())
                    .unwrap_or_else(|| "None".to_string()),
            )
            .field(
                "out_ty",
                &self
                    .out
                    .as_ref()
                    .map(|t| t.to_token_stream().to_string())
                    .unwrap_or_else(|| "None".to_string()),
            )
            .finish()
    }
}

/// Type of an arithmetic operation.
pub enum OpTy {
    Add,
    Sub,
    Mul,
    Div,
}

/// Returns a list of primitive types used to expand left-hand side
/// implementation.
pub fn scalars() -> Vec<String> {
    // FIXME: Remove harcoded 'T'.
    vec!["f64", "T"]
        .into_iter()
        .map(|s| String::from(s))
        .collect()
}

pub fn is_scalar(t: &TokenStream2) -> bool {
    scalars().iter().any(|s| *s == t.to_string())
}

/// An arithmetic operation.
pub struct Op {
    /// Token of the operation.
    op: TokenStream2,
    /// Trait name of the operation.
    op_trait: TokenStream2,
    /// Function name of the operation.
    op_fn: TokenStream2,
    /// Token of the assignment operation.
    op_assign: TokenStream2,
    /// Trait name of the assignment operation.
    op_assign_trait: TokenStream2,
    /// Function name of the assignment operation.
    op_assign_fn: TokenStream2,
}

impl Op {
    pub fn new(op_ty: OpTy) -> Self {
        match op_ty {
            OpTy::Add => {
                return Self {
                    op: quote! {+},
                    op_trait: quote! {std::ops::Add},
                    op_fn: quote! {add},
                    op_assign: quote! {+=},
                    op_assign_trait: quote! {std::ops::AddAssign},
                    op_assign_fn: quote! {add_assign},
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
    // This is not (should be):
    // struct Test<T: Clone + Copy>

    /// Expands an arithmetic operation definition into tokens.
    pub fn expand(
        &mut self,
        attr: OpAttr,
        input: &mut StructArrStore,
    ) -> TokenStream2 {
        // if !input.is_named() {
        //     panic!("expected named struct");
        // }

        let mut where_clause = input.gens.make_where_clause().clone();
        let mut where_clause_assign = input.gens.make_where_clause().clone();
        let op_trait = &self.op_trait;
        let op_assign_trait = &self.op_assign_trait;

        let gen_param = input.get_ty_param_ident();

        if input.has_generic_ty() {
            where_clause.predicates.push(parse_quote! {
                #gen_param: #op_trait<Output = #gen_param> + Copy
            });

            where_clause_assign.predicates.push(parse_quote! {
                #gen_param: #op_assign_trait + Copy
            });
        }

        let name = &input.name;
        let (impl_gens, ty_gens, _) = input.gens.split_for_impl();

        let op = &self.op;

        let self_ty = quote! {#name #ty_gens};
        let out = &attr.out.unwrap_or(self_ty.clone());

        type TS2 = TokenStream2;

        // Expands the arithmetic operation.
        let expand_op = |lhs: &TS2, rhs: &TS2, out: &TS2| -> TS2 {
            let lhs_tok = if is_scalar(&lhs) {
                quote! {self}
            } else {
                quote! {self.arr[i]}
            };

            let rhs_tok = if is_scalar(&rhs) {
                quote! {rhs}
            } else {
                quote! {rhs.arr[i]}
            };

            let output = if is_scalar(out) {
                // FIXME: I am not sure about this.
                panic!("expected output to be of type 'ArrStore'");
            } else {
                let out_init = {
                    let out_ty: Type = parse2(out.clone())
                        .expect("expected valid output type");

                    if let Type::Path(ty_path) = out_ty {
                        ty_path.path.segments.first().unwrap().ident.clone()
                    } else {
                        panic!("expected valid output type path");
                    }
                };
                quote! {
                    #out_init {
                        arr: ArrStore(
                            std::array::from_fn(|i| #lhs_tok #op #rhs_tok)
                        )
                    }
                }
            };

            // Replaces the generic type parameter of a ArrStore struct with a
            // concrete scalar type.
            let replace_gen_ty_with_scalar =
                |t: &TokenStream2| -> TokenStream2 {
                    let s = t.to_string();

                    if let (Some(start), Some(end)) = (s.find("<"), s.find(">"))
                    {
                        // Includes the '<'.
                        let before = &s[..=start];

                        // Between < and >.
                        let inside = &s[start + 1..end];

                        // includes the '>'.
                        let after = &s[end..];

                        let replaced_inside = inside
                            .replace(&gen_param.to_string(), &lhs.to_string());

                        let replaced =
                            format!("{}{}{}", before, replaced_inside, after);

                        parse_str(&replaced).unwrap()
                    } else {
                        panic!("expected generic params");
                    }
                };

            let mut gens = input.gens.clone();
            let mut where_clause = where_clause.clone();

            let mut rhs = rhs.clone();
            let mut out = out.clone();

            if is_scalar(&lhs) && input.has_generic_ty() {
                rhs = replace_gen_ty_with_scalar(&self_ty);
                out = replace_gen_ty_with_scalar(&out);

                // Remove generic type parameter from impl.
                gens.params = gens
                    .params
                    .into_iter()
                    .filter(|p| !matches!(p, GenericParam::Type(_)))
                    .collect::<Punctuated<_, Comma>>();

                // Remove generic type parameter from where clause.
                where_clause.predicates = where_clause
                    .predicates
                    .into_iter()
                    .filter(|pred| {
                        let s = pred.to_token_stream().to_string();
                        !s.starts_with(&gen_param.to_string())
                    })
                    .collect();
            }

            let (impl_gens, _, _) = gens.split_for_impl();
            let op_fn = &self.op_fn;

            quote! {
                impl #impl_gens #op_trait<#rhs> for #lhs #where_clause {
                    type Output = #out;
                    fn #op_fn(self, rhs: #rhs) -> Self::Output {
                        #output
                    }
                }
            }
        };

        let op_assign = &self.op_assign;
        let op_assign_fn = &self.op_assign_fn;

        // Expands the assignment operation.
        let expand_op_assign = |lhs: &TS2, rhs: &TS2| -> TS2 {
            if is_scalar(lhs) {
                panic!("expected lhs to be of type 'ArrStore'");
            }

            let output = if is_scalar(rhs) {
                quote! {
                    for mut val in self.arr.0.iter_mut() {
                        *val #op_assign rhs
                    }
                }
            } else {
                quote! {
                    for (i, mut val) in self.arr.0.iter_mut().enumerate() {
                        *val #op_assign rhs.arr[i]
                    }
                }
            };

            quote! {
                impl #impl_gens #op_assign_trait<#rhs> for #lhs
                #where_clause_assign {
                    fn #op_assign_fn(&mut self, rhs: #rhs) {
                        #output
                    }
                }
            }
        };

        // Expands a left-hand side operation for each supported scalar type.
        let expand_lhs_scalars_for = |rhs: &TS2| {
            let mut expanded_scalars = quote! {};

            for scalar in scalars() {
                if scalar == gen_param.to_string().as_str() {
                    continue;
                }

                let lhs_ty: Type = parse_str(&scalar).unwrap();
                let lhs: TokenStream2 = parse_quote! {#lhs_ty};
                expanded_scalars.extend(expand_op(&lhs, rhs, out));
            }

            expanded_scalars
        };

        let mut expanded = quote! {};

        // The redundancy here is deliberate to improve readability.

        if attr.lhs.is_none() && attr.rhs.is_none() {
            expanded.extend(expand_op(&self_ty, &self_ty, &out));

            if out.to_string() == self_ty.to_string() {
                expanded.extend(expand_op_assign(&self_ty, &self_ty));
            }
        }

        if attr.lhs.is_none() && !attr.rhs.is_none() {
            let rhs = attr.rhs.clone().unwrap();
            expanded.extend(expand_op(&self_ty, &rhs, &out));

            if out.to_string() == self_ty.to_string() {
                expanded.extend(expand_op_assign(&self_ty, &rhs));
            }
        }

        if !attr.lhs.is_none() && attr.rhs.is_none() {
            let lhs = attr.lhs.clone().unwrap();

            if is_scalar(&lhs) {
                expanded.extend(expand_lhs_scalars_for(&self_ty));
            } else {
                expanded.extend(expand_op(&lhs, &self_ty, &out));
            }
        }

        if !attr.lhs.is_none() && !attr.rhs.is_none() {
            let lhs = attr.lhs.clone().unwrap();
            let rhs = attr.rhs.clone().unwrap();

            if is_scalar(&lhs) {
                expanded.extend(expand_lhs_scalars_for(&self_ty));
            } else {
                expanded.extend(expand_op(&lhs, &self_ty, &out));
            }

            expanded.extend(expand_op(&self_ty, &rhs, &out));

            if out.to_string() == self_ty.to_string() {
                expanded.extend(expand_op_assign(&self_ty, &rhs));
            }
        }

        TokenStream2::from(expanded)
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
