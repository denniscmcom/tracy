use quote::quote;
use syn::{
    ConstParam, Data, DeriveInput, Error, Field, Fields, Generics, Ident, Type,
    TypeParam,
    parse::{Parse, ParseStream},
    parse_str,
    spanned::Spanned,
};

use crate::utils::get_ident;

///
/// Represents a struct with an `ArrStore` member that may have generic type
/// and/or const parameters.
///
/// Example forms of structs that can be parsed into `StructArrStore`:
///
/// Generic type and const parameter:
/// ```
/// struct S<T: Num, N: const usize> {
///     arr: ArrStore<T, N>,
/// }
/// ```
///
/// Generic type only:
/// ```
/// struct S<T: Num> {
///     arr: ArrStore<T, 5>,
/// }
/// ```
///
/// Const parameter only:
/// ```
/// struct S<N: const usize> {
///     arr: ArrStore<f64, N>,
/// }
/// ```
///
/// Fully concrete:
/// ```
/// struct S {
///     arr: ArrStore<f64, 5>,
/// }
/// ```
///
/// Tuple struct with a generic or concrete type:
/// ```
/// struct S<T: Num>(T);
/// struct S(f64);
/// ```

pub struct StructArrStore {
    /// The name of the struct.
    pub name: Ident,
    /// The generics attached to the struct.
    pub gens: Generics,
    /// The form of the struct.
    pub form: Form,
    /// The field of the struct.
    pub field: Field,
    /// The AST received by the macro.
    pub ast: DeriveInput,
}

/// Represents the form of the structure.
pub enum Form {
    /// Named structure with generic type and const parameters.
    GenericConst(TypeParam, ConstParam),
    /// Named structure with generic type and concrete 'ArrStore' length.
    Generic(TypeParam, usize),
    /// Named structure with concrete type and generic const parameter.
    Const(Type, ConstParam),
    /// Named structure with concrete type and 'ArrStore' length.
    Concrete(Type, usize),
    /// Tuple structure with generic type.
    TupleGeneric(TypeParam),
    /// Tuple structure with concrete type.
    TupleConcrete(Type),
}

impl Parse for StructArrStore {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ast: DeriveInput = input.parse()?;
        let gens = ast.generics.clone();

        let data_struct = match &ast.data {
            Data::Struct(data_struct) => data_struct,
            _ => return Err(Error::new(ast.span(), "expected struct")),
        };

        let ty_param = if gens.type_params().count() > 1 {
            return Err(Error::new(
                ast.span(),
                "expected zero or one generic type parameter",
            ));
        } else {
            gens.type_params().next().cloned()
        };

        let const_param = if gens.const_params().count() > 1 {
            return Err(Error::new(
                ast.span(),
                "expected zero or one generic const parameter",
            ));
        } else {
            gens.const_params().next().cloned()
        };

        let (field, form): (Field, Form) = match &data_struct.fields {
            Fields::Named(fields) => {
                if fields.named.iter().count() != 1 {
                    return Err(Error::new(ast.span(), "expected one fields"));
                }

                let field = fields.named.first().unwrap().clone();
                let field_ident = field.ident.as_ref().unwrap();

                if field_ident != "arr" {
                    return Err(Error::new(
                        ast.span(),
                        "expected one field named 'arr'",
                    ));
                }

                let field_ty = field.ty.clone();
                let field_ty_str = quote! {#field_ty}.to_string();

                if !field_ty_str.starts_with("ArrStore") {
                    return Err(Error::new(
                        ast.span(),
                        "expected 'arr' to have type 'ArrStore'",
                    ));
                }

                let start_params = field_ty_str.find('<').unwrap();
                let end_params = field_ty_str.find('>').unwrap();
                let params: Vec<&str> = field_ty_str
                    [start_params + 1..end_params]
                    .split(',')
                    .map(|s| s.trim())
                    .collect();

                let arr_store_t = params[0];
                let arr_store_n = params[1];

                if let Some(ty_param) = ty_param {
                    if let Some(const_param) = const_param {
                        (field, Form::GenericConst(ty_param, const_param))
                    } else {
                        let arr_store_len: usize = arr_store_n.parse().unwrap();
                        (field, Form::Generic(ty_param, arr_store_len))
                    }
                } else {
                    let arr_store_ty: Type = parse_str(arr_store_t).unwrap();

                    if let Some(const_param) = const_param {
                        (field, Form::Const(arr_store_ty, const_param))
                    } else {
                        let arr_store_len: usize = arr_store_n.parse().unwrap();
                        (field, Form::Concrete(arr_store_ty, arr_store_len))
                    }
                }
            }
            Fields::Unnamed(fields) => {
                if fields.unnamed.iter().count() != 1 {
                    return Err(Error::new(ast.span(), "expected one field"));
                }

                let field = fields.unnamed.first().unwrap().clone();
                let field_ty = field.ty.clone();

                let ty_param = if gens.type_params().count() > 1 {
                    return Err(Error::new(
                        ast.span(),
                        "expected zero or one generic type parameter",
                    ));
                } else {
                    gens.type_params().next().cloned()
                };

                if gens.const_params().count() > 1 {
                    return Err(Error::new(
                        ast.span(),
                        "expected zero const parameter",
                    ));
                };

                if let Some(ty_param) = ty_param {
                    (field, Form::TupleGeneric(ty_param))
                } else {
                    (field, Form::TupleConcrete(field_ty))
                }
            }
            Fields::Unit => {
                return Err(Error::new(
                    ast.span(),
                    "expected named or unnamed fields",
                ));
            }
        };

        Ok(Self {
            name: ast.ident.clone(),
            gens,
            form,
            field,
            ast,
        })
    }
}

impl StructArrStore {
    /// Checks if the struct has named fields.
    pub fn is_named(&self) -> bool {
        if let Form::GenericConst(_, _)
        | Form::Generic(_, _)
        | Form::Const(_, _)
        | Form::Concrete(_, _) = self.form
        {
            return true;
        }

        false
    }

    /// Checks if the struct has a generic type parameter.
    pub fn has_generic_ty(&self) -> bool {
        if let Form::GenericConst(_, _)
        | Form::Generic(_, _)
        | Form::TupleGeneric(_) = self.form
        {
            return true;
        }

        false
    }

    /// Checks if the struct has a generic const parameter.
    pub fn has_generic_const(&self) -> bool {
        if let Form::GenericConst(_, _) | Form::Const(_, _) = self.form {
            return true;
        }

        false
    }

    /// Returns the identifier of the type parameter, generic or concrete.
    pub fn get_ty_param_ident(&self) -> Ident {
        match &self.form {
            Form::GenericConst(ty_param, _)
            | Form::Generic(ty_param, _)
            | Form::TupleGeneric(ty_param) => ty_param.ident.clone(),

            Form::Const(ty, _)
            | Form::Concrete(ty, _)
            | Form::TupleConcrete(ty) => get_ident(&ty).unwrap(),
        }
    }
}
