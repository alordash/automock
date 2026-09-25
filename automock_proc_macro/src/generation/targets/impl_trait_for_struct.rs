mod as_trait_control_impl;

use crate::common::models::*;
use crate::common::*;
use crate::generation::mock_controls::models::*;
use crate::generation::mock_controls::*;
use crate::generation::mock_struct::models::*;
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
    let Some((trait_path, _)) = &item_impl.trait_ else {
        panic!("When mocking trait implementation `item_impl` must have trait field.")
    };
    let impl_trait_for_struct_syntax =
        impl_trait_for_struct_syntax::prepare(impl_trait_for_struct_syntax::Params {
            attributes: item_impl.attrs.clone(),
            merged_generics: item_impl.generics.clone(),
            target_type: item_impl.self_ty.clone(),
            trait_path: trait_path.clone(),
            impl_items: item_impl.items.clone(),
        });
    let impl_trait_for_struct_info =
        impl_trait_for_struct_info::generate(ctx, impl_trait_for_struct_syntax);
    let control_struct_path =
        patch_lifetime::prepend_to_path(impl_trait_for_struct_info.target_path.clone());
    let control_struct_generics =
        patch_lifetime::prepend_to_generics(impl_trait_for_struct_info.merged_generics.clone());
    let control_struct_impl_generics = patch_lifetime::prepend_to_generics(
        impl_trait_for_struct_info.target_simple_generics.clone(),
    );

    let maybe_associated_controls =
        (!impl_trait_for_struct_info.associated_fns.is_empty()).then(|| {
            let trait_setup_struct = setup::generate(
                ctx,
                source_span,
                setup::Params {
                    ident: path::last_ident(&impl_trait_for_struct_info.target_path).clone(),
                    generics: control_struct_generics.clone(),
                    generics_for_impl: control_struct_generics.clone(),
                    mock_struct_path: &impl_trait_for_struct_info.target_path,
                    fn_infos: &impl_trait_for_struct_info.associated_fns,
                    maybe_trait_ident: Some(impl_trait_for_struct_info.trait_ident.clone()),
                },
            );
            let trait_received_struct = received::generate(
                ctx,
                source_span,
                received::Params {
                    ident: path::last_ident(&impl_trait_for_struct_info.target_path).clone(),
                    generics: control_struct_generics.clone(),
                    generics_for_impl: control_struct_generics.clone(),
                    mock_struct_path: &impl_trait_for_struct_info.target_path,
                    fn_infos: &impl_trait_for_struct_info.associated_fns,
                    maybe_trait_ident: Some(impl_trait_for_struct_info.trait_ident.clone()),
                },
            );
            let setup_struct_impl = as_trait_control_impl::generate(
                source_span,
                as_trait_control_impl::Params {
                    struct_path: &control_struct_path,
                    struct_generics: control_struct_impl_generics.clone(),
                    trait_control_struct_path: &trait_setup_struct.path,
                    as_trait_where_predicates: impl_trait_for_struct_info
                        .as_trait_where_predicates
                        .as_ref(),
                    trait_ident: &impl_trait_for_struct_info.trait_ident,
                    trait_generics: impl_trait_for_struct_info.trait_simple_generics.clone(),
                    maybe_common_where_clause: impl_trait_for_struct_info
                        .merged_generics
                        .where_clause
                        .clone(),
                    control_type: ControlType::Setup,
                    is_static: false,
                },
            );
            let received_struct_impl = as_trait_control_impl::generate(
                source_span,
                as_trait_control_impl::Params {
                    struct_path: &control_struct_path,
                    struct_generics: control_struct_impl_generics.clone(),
                    trait_control_struct_path: &trait_received_struct.path,
                    as_trait_where_predicates: impl_trait_for_struct_info
                        .as_trait_where_predicates
                        .as_ref(),
                    trait_ident: &impl_trait_for_struct_info.trait_ident,
                    trait_generics: impl_trait_for_struct_info.trait_simple_generics.clone(),
                    maybe_common_where_clause: impl_trait_for_struct_info
                        .merged_generics
                        .where_clause
                        .clone(),
                    control_type: ControlType::Received,
                    is_static: false,
                },
            );
            let associated_controls = TraitAssociatedControls {
                trait_setup_struct,
                trait_received_struct,
                setup_struct_impl,
                received_struct_impl,
            };
            return associated_controls;
        });
    let maybe_static_controls = (!impl_trait_for_struct_info.static_fns.is_empty()).then(|| {
        let trait_static_setup_struct = static_setup::generate(
            ctx,
            source_span,
            static_setup::Params {
                ident: path::last_ident(&impl_trait_for_struct_info.target_path).clone(),
                generics: control_struct_generics.clone(),
                generics_for_impl: control_struct_generics.clone(),
                maybe_argument_types: None,
                mock_struct_path: &impl_trait_for_struct_info.target_path,
                fn_infos: &impl_trait_for_struct_info.static_fns,
                for_static_fn: false,
                maybe_trait_ident: Some(impl_trait_for_struct_info.trait_ident.clone()),
            },
        );
        let trait_static_received_struct = static_received::generate(
            ctx,
            source_span,
            static_received::Params {
                ident: path::last_ident(&impl_trait_for_struct_info.target_path).clone(),
                generics: control_struct_generics.clone(),
                generics_for_impl: control_struct_generics,
                maybe_argument_types: None,
                mock_struct_path: &impl_trait_for_struct_info.target_path,
                fn_infos: &impl_trait_for_struct_info.static_fns,
                for_static_fn: false,
                maybe_trait_ident: Some(impl_trait_for_struct_info.trait_ident.clone()),
            },
        );
        let static_setup_struct_impl = as_trait_control_impl::generate(
            source_span,
            as_trait_control_impl::Params {
                struct_path: &control_struct_path,
                struct_generics: control_struct_impl_generics.clone(),
                trait_control_struct_path: &trait_static_setup_struct.path,
                as_trait_where_predicates: impl_trait_for_struct_info
                    .as_trait_where_predicates
                    .as_ref(),
                trait_ident: &impl_trait_for_struct_info.trait_ident,
                trait_generics: impl_trait_for_struct_info.trait_simple_generics.clone(),
                maybe_common_where_clause: impl_trait_for_struct_info
                    .merged_generics
                    .where_clause
                    .clone(),
                control_type: ControlType::Setup,
                is_static: true,
            },
        );
        let static_received_struct_impl = as_trait_control_impl::generate(
            source_span,
            as_trait_control_impl::Params {
                struct_path: &control_struct_path,
                struct_generics: control_struct_impl_generics,
                trait_control_struct_path: &trait_static_received_struct.path,
                as_trait_where_predicates: impl_trait_for_struct_info
                    .as_trait_where_predicates
                    .as_ref(),
                trait_ident: &impl_trait_for_struct_info.trait_ident,
                trait_generics: impl_trait_for_struct_info.trait_simple_generics.clone(),
                maybe_common_where_clause: impl_trait_for_struct_info
                    .merged_generics
                    .where_clause
                    .clone(),
                control_type: ControlType::Received,
                is_static: true,
            },
        );
        let static_controls = TraitStaticControls {
            trait_static_setup_struct,
            trait_static_received_struct,
            static_setup_struct_impl,
            static_received_struct_impl,
        };
        return static_controls;
    });
    source_static_fn_block::replace(
        source_span,
        &impl_trait_for_struct_info.target_path,
        &mut item_impl,
        Some(impl_trait_for_struct_info.trait_path.clone()),
    );
    let call_site = proc_macro::Span::call_site();
    let line = call_site.line();
    let column = call_site.column();
    let mod_ident = format_ident!(
        "__automock_generated_{}_{}_{}",
        path::last_ident(&impl_trait_for_struct_info.target_path),
        line,
        column
    );
    let mock_struct_impls = mock_struct_impl::generate_for_trait(
        ctx,
        source_span,
        mock_struct_impl::ParamsForTrait {
            attributes: impl_trait_for_struct_info.attributes,
            unsafety: item_impl.unsafety,
            mock_struct_path: impl_trait_for_struct_info.target_path.clone(),
            constants: &impl_trait_for_struct_info.constants,
            types: &impl_trait_for_struct_info.types,
            associated_fns: &impl_trait_for_struct_info.associated_fns,
            static_fns: &impl_trait_for_struct_info.static_fns,
            merged_generics: impl_trait_for_struct_info.merged_generics,
            trait_path: impl_trait_for_struct_info.trait_path.clone(),
            mod_ident: &mod_ident,
        },
    );

    let use_struct_mod =
        use_struct_mod::generate(source_span, &impl_trait_for_struct_info.target_path);
    let mock_mod_usages = mock_mod_usages::new(source_span);
    let items = [
        Item::Use(mock_mod_usages.use_super),
        Item::Use(use_struct_mod),
    ]
    .into_iter()
    .chain(
        impl_trait_for_struct_info
            .associated_fns
            .into_iter()
            .flat_map(|x| {
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
            }),
    )
    .chain(
        impl_trait_for_struct_info
            .static_fns
            .into_iter()
            .flat_map(|x| {
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
            }),
    )
    .chain(
        mock_struct_impls
            .maybe_base_trait_and_fns_impl
            .map(|(base_trait, base_fns_impl)| [Item::Trait(base_trait), Item::Impl(base_fns_impl)])
            .into_iter()
            .flatten(),
    )
    .chain(maybe_associated_controls.into_iter().flat_map(|x| {
        [
            Item::Struct(x.trait_setup_struct.item_struct),
            Item::Impl(x.trait_setup_struct.item_impl),
            Item::Struct(x.trait_received_struct.item_struct),
            Item::Impl(x.trait_received_struct.clone_impl),
            Item::Impl(x.trait_received_struct.item_impl),
            Item::Impl(x.setup_struct_impl),
            Item::Impl(x.received_struct_impl),
        ]
    }))
    .chain(maybe_static_controls.into_iter().flat_map(|x| {
        [
            Item::Struct(x.trait_static_setup_struct.item_struct),
            Item::Impl(x.trait_static_setup_struct.item_impl),
            Item::Struct(x.trait_static_received_struct.item_struct),
            Item::Impl(x.trait_static_received_struct.clone_impl),
            Item::Impl(x.trait_static_received_struct.item_impl),
            Item::Impl(x.static_setup_struct_impl),
            Item::Impl(x.static_received_struct_impl),
        ]
    }))
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
        source_item: Item::Impl(mock_struct_impls.trait_impl),
        maybe_usage: None,
        item_mod,
    };
    return result;
}
