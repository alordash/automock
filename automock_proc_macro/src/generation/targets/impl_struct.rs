use crate::common::models::*;
use crate::common::*;
use crate::generation::mock_controls::*;
use crate::generation::targets::common::*;
use crate::generation::targets::models::*;
use crate::generation::targets::*;
use crate::generation::*;
use crate::preparation::r#struct::*;
use not_enough_syntax::*;
use quote::format_ident;
use syn::spanned::Spanned;
use syn::*;

pub(crate) fn generate_module(ctx: &Context, mut item_impl: ItemImpl) -> MockMod {
    let source_span = item_impl.span();
    let impl_struct_syntax = impl_struct_syntax::prepare(impl_struct_syntax::Params {
        attributes: item_impl.attrs.clone(),
        generics: item_impl.generics.clone(),
        target_type: item_impl.self_ty.clone(),
        impl_items: item_impl.items.clone(),
    });
    let impl_struct_info = impl_struct_info::generate(ctx, impl_struct_syntax);
    let generics_for_impl =
        patch_lifetime::prepend_to_generics(impl_struct_info.generics.clone());
    let call_site = proc_macro::Span::call_site();
    let line = call_site.line();
    let column = call_site.column();
    let mod_ident = format_ident!(
        "__automock_generated_{}_{}_{}",
        impl_struct_info.target_ident,
        line,
        column
    );
    let mock_struct_impl = mock_struct_impl::generate(
        ctx,
        source_span,
        mock_struct_impl::Params {
            attributes: impl_struct_info.attributes,
            unsafety: item_impl.unsafety,
            mock_struct_path: impl_struct_info.target_path.clone(),
            associated_fns: &impl_struct_info.associated_fns,
            static_fns: &impl_struct_info.static_fns,
            generics: impl_struct_info.generics.clone(),
            mod_ident: &mod_ident,
            qualify_call: true,
        },
    );
    let control_struct_path_base =
        patch_lifetime::prepend_to_path(impl_struct_info.target_path.clone());
    let maybe_associated_controls_impls =
        (!impl_struct_info.associated_fns.is_empty()).then(|| {
            let setup_impl = setup_impl::generate(
                ctx,
                source_span,
                setup_impl::Params {
                    setup_struct_path: path::from_base_path_with_ident(
                        &control_struct_path_base,
                        format_ident!("{}Setup", path::last_ident(&control_struct_path_base)),
                    ),
                    generics: generics_for_impl.clone(),
                    mock_struct_path: &impl_struct_info.target_path,
                    fn_infos: &impl_struct_info.associated_fns,
                    for_static_fn: false,
                    is_static: false,
                },
            );
            let received_impl = received_impl::generate(
                ctx,
                source_span,
                received_impl::Params {
                    received_struct_path: path::from_base_path_with_ident(
                        &control_struct_path_base,
                        format_ident!("{}Received", path::last_ident(&control_struct_path_base)),
                    ),
                    generics: generics_for_impl.clone(),
                    mock_struct_path: &impl_struct_info.target_path,
                    fn_infos: &impl_struct_info.associated_fns,
                    for_static_fn: false,
                    is_static: false,
                    generate_fn_no_other_calls: false,
                },
            );
            (setup_impl, received_impl)
        });
    let maybe_static_controls_impls = (!impl_struct_info.static_fns.is_empty()).then(|| {
        let static_setup_impl = setup_impl::generate(
            ctx,
            source_span,
            setup_impl::Params {
                setup_struct_path: path::from_base_path_with_ident(
                    &control_struct_path_base,
                    format_ident!("{}StaticSetup", path::last_ident(&control_struct_path_base)),
                ),
                generics: generics_for_impl.clone(),
                mock_struct_path: &impl_struct_info.target_path,
                fn_infos: &impl_struct_info.static_fns,
                for_static_fn: false,
                is_static: true,
            },
        );
        let static_received_impl = received_impl::generate(
            ctx,
            source_span,
            received_impl::Params {
                received_struct_path: path::from_base_path_with_ident(
                    &control_struct_path_base,
                    format_ident!(
                        "{}StaticReceived",
                        path::last_ident(&control_struct_path_base)
                    ),
                ),
                generics: generics_for_impl,
                mock_struct_path: &impl_struct_info.target_path,
                fn_infos: &impl_struct_info.static_fns,
                for_static_fn: false,
                is_static: true,
                generate_fn_no_other_calls: false,
            },
        );
        (static_setup_impl, static_received_impl)
    });
    source_static_fn_block::replace(
        source_span,
        &impl_struct_info.target_path,
        &mut item_impl,
        None,
    );

    let use_struct_mod = use_struct_mod::generate(source_span, &impl_struct_info.target_path);
    let mock_mod_usages = mock_mod_usages::new(source_span);
    let items = [
        Item::Use(mock_mod_usages.use_super),
        Item::Use(use_struct_mod),
    ]
    .into_iter()
    .chain(impl_struct_info.associated_fns.into_iter().flat_map(|x| {
        let call_struct = x.value.call_struct;
        let args_checker = x.value.args_checker_struct;
        [
            Item::Struct(call_struct.item_struct),
            Item::Impl(call_struct.generics_info_provider_impl),
            Item::Impl(call_struct.call_impl),
        ]
        .into_iter()
        .chain(call_struct.maybe_clone_impl.map(Item::Impl))
        .chain([
            Item::Struct(args_checker.item_struct),
            Item::Impl(args_checker.generics_info_provider_impl),
            Item::Impl(args_checker.args_checker_impl),
        ])
    }))
    .chain(impl_struct_info.static_fns.into_iter().flat_map(|x| {
        let call_struct = x.value.call_struct;
        let args_checker = x.value.args_checker_struct;
        [
            Item::Struct(call_struct.item_struct),
            Item::Impl(call_struct.generics_info_provider_impl),
            Item::Impl(call_struct.call_impl),
        ]
        .into_iter()
        .chain(call_struct.maybe_clone_impl.map(Item::Impl))
        .chain([
            Item::Struct(args_checker.item_struct),
            Item::Impl(args_checker.generics_info_provider_impl),
            Item::Impl(args_checker.args_checker_impl),
        ])
    }))
    .chain(
        maybe_associated_controls_impls
            .into_iter()
            .flat_map(|x| [Item::Impl(x.0), Item::Impl(x.1)]),
    )
    .chain(
        maybe_static_controls_impls
            .into_iter()
            .flat_map(|x| [Item::Impl(x.0), Item::Impl(x.1)]),
    )
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
        vis: Visibility::Public(Token![pub](source_span)),
        unsafety: None,
        mod_token: Token![mod](source_span),
        ident: mod_ident,
        content: Some((token::Brace(source_span), items)),
        semi: None,
    };
    let result = MockMod {
        source_item: Item::Impl(mock_struct_impl),
        maybe_usage: None,
        item_mod,
    };
    return result;
}
