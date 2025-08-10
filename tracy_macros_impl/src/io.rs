use syn::{
    DeriveInput, Ident,
    parse::{Parse, ParseStream},
};

pub struct Input {
    pub ast: DeriveInput,
    pub ident: Ident,
    pub fields_idents: Vec<Ident>,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ast: DeriveInput = input.parse()?;
        let ident = ast.ident.clone();
        let fields = match &ast.data {
            syn::Data::Struct(data_struct) => &data_struct.fields,
            _ => panic!("Macro only supports structs"),
        };

        let fields_idents: Vec<_> = match fields {
            syn::Fields::Named(named_fields) => named_fields
                .named
                .iter()
                .map(|f| f.ident.clone().unwrap())
                .collect(),
            _ => panic!("Macro only supports structs with named fields"),
        };

        Ok(Self {
            ast,
            ident,
            fields_idents,
        })
    }
}
