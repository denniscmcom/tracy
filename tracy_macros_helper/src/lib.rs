use syn::{
    Data, DeriveInput, Fields, Ident, Member,
    parse::{Parse, ParseStream},
};

pub struct StructInput {
    pub ast: DeriveInput,
    pub ident: Ident,
    pub fields: Vec<Member>,
}

impl Parse for StructInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ast: DeriveInput = input.parse()?;
        let ident = ast.ident.clone();

        let fields: Vec<Member> = if let Data::Struct(data_struct) = &ast.data {
            match &data_struct.fields {
                Fields::Named(fields_named) => fields_named
                    .named
                    .iter()
                    .map(|f| Member::Named(f.ident.clone().unwrap()))
                    .collect(),
                Fields::Unnamed(fields_unnamed) => fields_unnamed
                    .unnamed
                    .iter()
                    .enumerate()
                    .map(|(i, _f)| Member::Unnamed(syn::Index::from(i)))
                    .collect(),
                Fields::Unit => panic!("unit struct is not supported"),
            }
        } else {
            panic!("type is not a struct");
        };

        Ok(Self { ast, ident, fields })
    }
}
