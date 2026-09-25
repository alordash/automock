use crate::common::*;
use crate::generation::fn_info::models::*;
use crate::generation::mock_struct::models::*;
use crate::preparation::r#fn::models::*;
use not_enough_syntax::*;
use proc_macro2::Span;
use quote::format_ident;
use syn::*;

pub(crate) fn generate(span: Span, fn_info: &FnInfo) -> StaticFnMockStruct {
    let item_struct = ItemStruct {
        attrs: Vec::new(),
        vis: Visibility::Public(Token![pub](span)),
        struct_token: Token![struct](span),
        ident: format_ident!("{}Mock", fn_info.fn_ident),
        generics: fn_info.merged_generics.clone(),
        fields: Fields::Named(FieldsNamed {
            brace_token: token::Brace(span),
            named: punctuated([generics_field::new_field(
                span,
                &fn_info.merged_generics,
                Some(fn_info.arguments.iter_generics_style_types().collect()),
            )]),
        }),
        semi_token: None,
    };
    let path = path::from_ident_with_generics(item_struct.ident.clone(), &item_struct.generics);

    let result = StaticFnMockStruct { path, item_struct };
    return result;
}
