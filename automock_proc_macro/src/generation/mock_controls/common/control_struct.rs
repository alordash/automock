use crate::common::*;
use crate::generation::mock_controls::models::*;
use not_enough_syntax::*;
use proc_macro2::Span;
use quote::format_ident;
use syn::*;

pub(crate) fn new(
    span: Span,
    ident: Ident,
    generics: Generics,
    control_type: ControlType,
    maybe_trait_ident: Option<Ident>,
) -> ItemStruct {
    let ident_suffix = get_control_ident_suffix(control_type);
    let result_ident = if let Some(trait_ident) = maybe_trait_ident {
        format_ident!("{ident}{trait_ident}{ident_suffix}")
    } else {
        format_ident!("{ident}{ident_suffix}")
    };
    let result = ItemStruct {
        attrs: vec![attributes::doc_hidden(span)],
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
    control_type: ControlType,
    maybe_trait_ident: Option<Ident>,
) -> ItemStruct {
    let ident_suffix = get_control_ident_suffix(control_type);
    let result_ident = if let Some(trait_ident) = maybe_trait_ident {
        format_ident!("{ident}{trait_ident}Static{ident_suffix}")
    } else {
        format_ident!("{ident}Static{ident_suffix}")
    };
    let result = ItemStruct {
        attrs: vec![attributes::doc_hidden(span)],
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

fn get_control_ident_suffix(control_type: ControlType) -> &'static str {
    match control_type {
        ControlType::Setup => "Setup",
        ControlType::Received => "Received",
    }
}
