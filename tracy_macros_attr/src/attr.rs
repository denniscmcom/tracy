use syn::{
    Ident, Token, Type,
    parse::{Parse, ParseStream},
};

pub struct Attr {
    pub lhs_ty: Option<Type>,
    pub rhs_ty: Option<Type>,
    pub out_ty: Option<Type>,
}

impl Parse for Attr {
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
