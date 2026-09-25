mod call_impl;

use crate::common::models::*;
use crate::generation::common::*;
use crate::generation::fn_info::models::*;
use crate::generation::fn_info::*;
use crate::preparation::r#fn::models::*;
use not_enough_syntax::*;
use quote::format_ident;
use syn::*;

pub(crate) fn generate(
    ctx: &Context,
    fn_syntax: &FnSyntax,
    generics_for_impl: Generics,
) -> CallStruct {
    let span = fn_syntax.spans.inputs;
    let fields_named = generate_fields(fn_syntax);
    let struct_ident = format_ident!("{}_Call", fn_syntax.fn_ident);
    let generics = fn_syntax.merged_generics.clone();
    let path = path::from_ident_with_generics(struct_ident.clone(), &generics);

    let item_struct = ItemStruct {
        attrs: vec![attributes::doc_hidden(span)],
        vis: Visibility::Public(Token![pub](span)),
        struct_token: Token![struct](span),
        ident: struct_ident.clone(),
        generics,
        fields: Fields::Named(fields_named),
        semi_token: None,
    };
    let maybe_clone_impl = if ctx.support_base_calling && fn_syntax.maybe_base_impl.is_some() {
        Some(clone_impl::generate(
            span,
            generics_for_impl.clone(),
            path.clone(),
            &item_struct.fields,
        ))
    } else {
        None
    };

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
    let call_impl = call_impl::generate(
        span,
        generics_for_impl.clone(),
        &fn_syntax.arguments,
        r#type.clone(),
    );

    let result = CallStruct {
        path,
        item_struct,
        generics_info_provider_impl,
        call_impl,
        maybe_clone_impl,
    };

    return result;
}

fn generate_fields(fn_syntax: &FnSyntax) -> FieldsNamed {
    let result = FieldsNamed {
        brace_token: token::Brace(fn_syntax.spans.inputs),
        named: core::iter::once(fn_syntax.generics_field.clone())
            .chain(fn_syntax.arguments.iter().map(|argument| {
                let span = argument.ident.span();
                let result = Field {
                    attrs: Vec::new(),
                    vis: visibility::pub_super(span),
                    modifiers: FieldModifiers::default(),
                    ident: Some(argument.ident.clone()),
                    colon_token: Some(Token![:](span)),
                    ty: *argument.ptr_style_type.clone(),
                    default: None,
                };

                return result;
            }))
            .collect(),
    };

    return result;
}
