mod mocked_fn;

use crate::common::models::*;
use crate::generation::mock_controls::*;
use crate::generation::mock_struct::*;
use crate::generation::targets::models::*;
use crate::generation::targets::*;
use crate::generation::*;
use crate::preparation::r#fn::fn_syntax;
use crate::preparation::r#fn::models::*;
use not_enough_syntax::attributes;
use syn::spanned::Spanned;
use syn::*;

pub(crate) fn generate_module(ctx: &Context, item_fn: ItemFn) -> MockMod {
    let source_span = item_fn.span();
    let fn_syntax = fn_syntax::prepare(fn_syntax::Params {
        attributes: item_fn.attrs,
        visibility: item_fn.vis,
        signature: item_fn.sig,
        maybe_base_impl: Some(item_fn.block),
        maybe_owner: None,
        maybe_target_path: None,
    });
    let fn_info = fn_info::generate(ctx, fn_syntax);

    let static_fn_mock_struct = static_fn_mock_struct::generate(source_span, &fn_info);

    let maybe_base_fn = if ctx.support_base_calling {
        let base_impl = fn_info
            .maybe_base_impl
            .clone()
            .expect("Static `fn`s should always have base implementation (body).");
        Some(base_fn::generate_static_fn(
            source_span,
            base_fn::StaticFnParams {
                fn_info: &fn_info,
                base_impl,
            },
        ))
    } else {
        None
    };
    let target_ident = fn_info.fn_ident.clone();
    let target_generics = fn_info.merged_generics.clone();
    let target_argument_types: Vec<_> = fn_info.arguments.iter_generics_style_types().collect();
    let fn_infos = [fn_info];
    let static_setup_struct = static_setup::generate(
        ctx,
        source_span,
        static_setup::Params {
            ident: target_ident.clone(),
            generics: target_generics.clone(),
            generics_for_impl: target_generics.clone(),
            maybe_argument_types: Some(target_argument_types.clone()),
            mock_struct_path: &static_fn_mock_struct.path,
            fn_infos: &fn_infos,
            for_static_fn: true,
            maybe_trait_ident: None,
        },
    );
    let static_received_struct = static_received::generate(
        ctx,
        source_span,
        static_received::Params {
            ident: target_ident,
            generics: target_generics.clone(),
            generics_for_impl: target_generics,
            maybe_argument_types: Some(target_argument_types),
            mock_struct_path: &static_fn_mock_struct.path,
            fn_infos: &fn_infos,
            for_static_fn: true,
            maybe_trait_ident: None,
        },
    );
    let [fn_info] = fn_infos;
    let fn_static_setup = fn_static_setup::generate(
        ctx,
        source_span,
        static_fn_mock_struct.path.clone(),
        static_setup_struct.path.clone(),
        &fn_info,
    );
    let fn_static_received =
        fn_static_received::generate(source_span, static_received_struct.path.clone(), &fn_info);
    let fn_static_received_nothing = fn_static_received_nothing::generate(
        source_span,
        static_fn_mock_struct.path.clone(),
        &fn_info,
    );

    let mod_ident = fn_info.signature.ident.clone();
    let mocked_fn = mocked_fn::generate(
        ctx,
        source_span,
        &fn_info,
        static_fn_mock_struct.path,
        mod_ident.clone(),
        maybe_base_fn.as_ref().map(|x| x.sig.ident.clone()),
    );

    let mock_mod_usages = mock_mod_usages::new(source_span);
    let items = [Item::Use(mock_mod_usages.use_super)]
        .into_iter()
        .chain(maybe_base_fn.map(Item::Fn))
        .chain([
            Item::Fn(fn_static_setup),
            Item::Fn(fn_static_received),
            Item::Fn(fn_static_received_nothing),
            Item::Struct(fn_info.call_struct.item_struct),
            Item::Impl(fn_info.call_struct.generics_info_provider_impl),
            Item::Impl(fn_info.call_struct.call_impl),
        ])
        .chain(fn_info.call_struct.maybe_clone_impl.map(Item::Impl))
        .chain([
            Item::Struct(fn_info.args_checker_struct.item_struct),
            Item::Impl(fn_info.args_checker_struct.generics_info_provider_impl),
            Item::Impl(fn_info.args_checker_struct.args_checker_impl),
            Item::Struct(static_fn_mock_struct.item_struct),
            Item::Struct(static_setup_struct.item_struct),
            Item::Impl(static_setup_struct.item_impl),
            Item::Struct(static_received_struct.item_struct),
            Item::Impl(static_received_struct.clone_impl),
            Item::Impl(static_received_struct.item_impl),
        ])
        .collect();

    let item_mod = ItemMod {
        attrs: vec![
            attributes::allow_clippy(source_span, "all"),
            attributes::allow_unused(source_span),
            attributes::allow_private_interfaces(source_span),
            attributes::allow_private_bounds(source_span),
            attributes::allow_unreachable_pub(source_span),
            attributes::allow_nonstandard_style(source_span),
        ],
        vis: fn_info.visibility.clone(),
        unsafety: None,
        mod_token: Token![mod](source_span),
        ident: mod_ident,
        content: Some((token::Brace(source_span), items)),
        semi: None,
    };
    let result = MockMod {
        source_item: Item::Fn(mocked_fn),
        maybe_usage: None,
        item_mod,
    };
    return result;
}
