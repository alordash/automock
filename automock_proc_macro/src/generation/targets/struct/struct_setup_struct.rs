use crate::common::*;
use not_enough_syntax::*;
use proc_macro2::Span;
use quote::format_ident;
use syn::*;

pub(crate) struct Params<'a> {
    pub struct_ident: &'a Ident,
    pub generics: Generics,
    pub is_static: bool,
}
pub(crate) fn generate(
    span: Span,
    Params {
        struct_ident,
        mut generics,
        is_static,
    }: Params,
) -> ItemStruct {
    let control_name = if is_static { "StaticSetup" } else { "Setup" };
    generics.where_clause = None;
    let generics_field = generics_field::new_field(span, &generics, None);
    let result = ItemStruct {
        attrs: Vec::new(),
        vis: Visibility::Public(Token![pub](span)),
        struct_token: Token![struct](span),
        ident: format_ident!("{struct_ident}{control_name}"),
        generics,
        fields: Fields::Named(FieldsNamed {
            brace_token: token::Brace(span),
            named: if is_static {
                punctuated([generics_field])
            } else {
                punctuated([generics_field, data_field::new_field(span)])
            },
        }),
        semi_token: None,
    };
    return result;
}
