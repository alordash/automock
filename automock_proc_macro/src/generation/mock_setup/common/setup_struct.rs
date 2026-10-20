use crate::common::*;
use not_enough_syntax::*;
use proc_macro2::Span;
use quote::format_ident;
use syn::*;

pub(crate) fn new(
    span: Span,
    ident: Ident,
    generics: Generics,
    maybe_trait_ident: Option<Ident>,
) -> ItemStruct {
    let result_ident = if let Some(trait_ident) = maybe_trait_ident {
        format_ident!("{ident}{trait_ident}Setup")
    } else {
        format_ident!("{ident}Setup")
    };
    let result = ItemStruct {
        attrs: Vec::new(),
        vis: Visibility::Public(Token![pub](span)),
        struct_token: Token![struct](span),
        ident: result_ident,
        generics: generics.clone(),
        fields: Fields::Named(FieldsNamed {
            brace_token: token::Brace(span),
            named: punctuated([
                generics_field::new_field(span, &generics, None),
                data_field::new_field(span),
            ]),
        }),
        semi_token: None,
    };
    return result;
}

pub(crate) fn new_static(
    span: Span,
    ident: Ident,
    generics: Generics,
    maybe_argument_types: Option<Vec<Type>>,
    maybe_trait_ident: Option<Ident>,
) -> ItemStruct {
    let result_ident = if let Some(trait_ident) = maybe_trait_ident {
        format_ident!("{ident}{trait_ident}StaticSetup")
    } else {
        format_ident!("{ident}StaticSetup")
    };
    let result = ItemStruct {
        attrs: Vec::new(),
        vis: Visibility::Public(Token![pub](span)),
        struct_token: Token![struct](span),
        ident: result_ident,
        generics: generics.clone(),
        fields: Fields::Named(FieldsNamed {
            brace_token: token::Brace(span),
            named: punctuated([generics_field::new_field(
                span,
                &generics,
                maybe_argument_types,
            )]),
        }),
        semi_token: None,
    };
    return result;
}
