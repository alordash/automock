use crate::common::models::*;
use crate::generation::common::*;
use crate::generation::mock_controls::*;
use crate::generation::mock_struct::models::*;
use crate::generation::mock_struct::*;
use crate::generation::targets::models::*;
use crate::generation::targets::*;
use crate::generation::*;
use crate::preparation::r#trait::*;
use not_enough_syntax::*;
use quote::format_ident;
use syn::spanned::Spanned;
use syn::*;

pub(crate) fn generate_module(ctx: &Context, item_trait: ItemTrait) -> MockMod {
    let source_span = item_trait.span();
    let trait_syntax = trait_syntax::prepare(trait_syntax::Params {
        unsafety: item_trait.unsafety,
        ident: item_trait.ident.clone(),
        generics: item_trait.generics.clone(),
        items: item_trait.items.clone(),
    });
    let trait_info = trait_info::generate(ctx, trait_syntax);
    let generics_for_impl = generics::remove_defaults(trait_info.merged_generics.clone());

    let trait_mock_struct_ident = format_ident!("{}Mock", trait_info.ident);
    let trait_mock_struct_path =
        path::from_ident_with_generics(trait_mock_struct_ident.clone(), &generics_for_impl);
    let maybe_associated_controls = (!trait_info.associated_fns.is_empty()).then(|| {
        let setup_struct = setup::generate(
            ctx,
            source_span,
            setup::Params {
                ident: trait_info.ident.clone(),
                generics: trait_info.merged_generics.clone(),
                generics_for_impl: generics_for_impl.clone(),
                mock_struct_path: &trait_mock_struct_path,
                fn_infos: &trait_info.associated_fns,
                maybe_trait_ident: None,
            },
        );
        let received_struct = received::generate(
            ctx,
            source_span,
            received::Params {
                ident: trait_info.ident.clone(),
                generics: trait_info.merged_generics.clone(),
                generics_for_impl: generics_for_impl.clone(),
                mock_struct_path: &trait_mock_struct_path,
                fn_infos: &trait_info.associated_fns,
                maybe_trait_ident: None,
            },
        );
        let associated_controls = AssociatedControls {
            setup_struct,
            received_struct,
        };
        return associated_controls;
    });
    let maybe_static_controls = (!trait_info.static_fns.is_empty()).then(|| {
        let static_setup_struct = static_setup::generate(
            ctx,
            source_span,
            static_setup::Params {
                ident: trait_info.ident.clone(),
                generics: trait_info.merged_generics.clone(),
                generics_for_impl: generics_for_impl.clone(),
                maybe_argument_types: None,
                mock_struct_path: &trait_mock_struct_path,
                fn_infos: &trait_info.static_fns,
                for_static_fn: false,
                maybe_trait_ident: None,
            },
        );
        let static_received_struct = static_received::generate(
            ctx,
            source_span,
            static_received::Params {
                ident: trait_info.ident.clone(),
                generics: trait_info.merged_generics.clone(),
                generics_for_impl: generics_for_impl.clone(),
                maybe_argument_types: None,
                mock_struct_path: &trait_mock_struct_path,
                fn_infos: &trait_info.static_fns,
                for_static_fn: false,
                maybe_trait_ident: None,
            },
        );
        let static_controls = StaticControls {
            static_setup_struct,
            static_received_struct,
        };
        return static_controls;
    });
    let mod_ident = format_ident!("__rsubstitute_generated_{}Mock", trait_info.ident);
    let trait_mock_struct = trait_mock_struct::generate(
        ctx,
        source_span,
        trait_mock_struct::Params {
            mock_struct_ident: trait_mock_struct_ident.clone(),
            trait_info: &trait_info,
            generics_for_impl,
            maybe_associated_controls,
            maybe_static_controls,
            mod_ident: &mod_ident,
        },
    );

    let mod_visibility = item_trait.vis.clone();
    let mock_mod_usages = mock_mod_usages::new(source_span);
    let items = [Item::Use(mock_mod_usages.use_super)]
        .into_iter()
        .chain(trait_info.associated_fns.into_iter().flat_map(|x| {
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
        .chain(trait_info.static_fns.into_iter().flat_map(|x| {
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
        .chain([
            Item::Struct(trait_mock_struct.item_struct),
            Item::Impl(trait_mock_struct.clone_impl),
            Item::Impl(trait_mock_struct.trait_impl),
            Item::Impl(trait_mock_struct.inner_impl),
        ])
        .chain(
            trait_mock_struct
                .maybe_associated_controls
                .into_iter()
                .flat_map(|associated_controls| {
                    [
                        Item::Struct(associated_controls.setup_struct.item_struct),
                        Item::Impl(associated_controls.setup_struct.item_impl),
                        Item::Struct(associated_controls.received_struct.item_struct),
                        Item::Impl(associated_controls.received_struct.clone_impl),
                        Item::Impl(associated_controls.received_struct.item_impl),
                    ]
                }),
        )
        .chain(
            trait_mock_struct
                .maybe_static_controls
                .into_iter()
                .flat_map(|static_controls| {
                    [
                        Item::Struct(static_controls.static_setup_struct.item_struct),
                        Item::Impl(static_controls.static_setup_struct.item_impl),
                        Item::Struct(static_controls.static_received_struct.item_struct),
                        Item::Impl(static_controls.static_received_struct.clone_impl),
                        Item::Impl(static_controls.static_received_struct.item_impl),
                    ]
                }),
        )
        .collect();

    let mut usage = mod_usage::new(
        item_trait.vis.clone(),
        mod_ident.clone(),
        [trait_mock_struct_ident],
    );
    usage.attrs.push(attributes::allow_unused(source_span));
    let item_mod = ItemMod {
        attrs: vec![
            attributes::allow_clippy(source_span, "all"),
            attributes::allow_unused(source_span),
            attributes::allow_private_interfaces(source_span),
            attributes::allow_private_bounds(source_span),
            attributes::allow_unreachable_pub(source_span),
            attributes::allow_nonstandard_style(source_span),
        ],
        vis: mod_visibility,
        unsafety: None,
        mod_token: Token![mod](source_span),
        ident: mod_ident,
        content: Some((token::Brace(source_span), items)),
        semi: None,
    };
    let result = MockMod {
        source_item: Item::Trait(item_trait),
        maybe_usage: Some(usage),
        item_mod,
    };
    return result;
}
