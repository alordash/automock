mod args_checker_impl;

use crate::generation::fn_info::models::*;
use crate::generation::fn_info::*;
use crate::preparation::r#fn::models::*;
use not_enough_syntax::{attributes, path};
use quote::format_ident;
use syn::*;

pub(crate) fn generate(
    fn_syntax: &FnSyntax,
    call_struct_type: Type,
    generics_for_impl: Generics,
) -> ArgsCheckerStruct {
    let span = fn_syntax.spans.inputs;
    let fields_named = generate_fields(fn_syntax);
    let item_struct = ItemStruct {
        attrs: vec![attributes::doc_hidden(span)],
        vis: Visibility::Inherited,
        struct_token: Token![struct](span),
        ident: format_ident!("{}_ArgsChecker", fn_syntax.fn_ident),
        generics: fn_syntax.merged_generics.clone(),
        fields: Fields::Named(fields_named),
        semi_token: None,
    };

    let path = path::from_ident_with_generics(item_struct.ident.clone(), &item_struct.generics);
    let r#type = Type::Path(TypePath {
        attrs: Vec::new(),
        qself: None,
        path: path.clone(),
    });
    let generics_info_provider_impl = generics_info_provider_impl::generate(
        generics_for_impl.clone(),
        fn_syntax.signature.generics.clone(),
        r#type.clone(),
    );
    let args_checker_impl = args_checker_impl::generate(
        span,
        generics_for_impl,
        &fn_syntax.arguments,
        r#type,
        call_struct_type,
    );

    let result = ArgsCheckerStruct {
        path,
        item_struct,
        generics_info_provider_impl,
        args_checker_impl,
    };

    return result;
}

fn generate_fields(fn_syntax: &FnSyntax) -> FieldsNamed {
    let result = FieldsNamed {
        brace_token: token::Brace(fn_syntax.spans.inputs),
        named: core::iter::once(fn_syntax.generics_field.clone())
            .chain(fn_syntax.arguments.iter().map(|argument| {
                let span = argument.ident.span();
                let ty = arg_type::of(span, *argument.ptr_style_type.clone());

                let result = Field {
                    attrs: Vec::new(),
                    vis: Visibility::Inherited,
                    modifiers: FieldModifiers::default(),
                    ident: Some(argument.ident.clone()),
                    colon_token: Some(Token![:](argument.ident.span())),
                    ty: Type::Path(ty),
                    default: None,
                };

                return result;
            }))
            .collect(),
    };

    return result;
}
