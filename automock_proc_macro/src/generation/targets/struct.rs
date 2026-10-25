mod mockable_trait_impl;
mod source_struct_fields;
mod struct_setup_struct;

use crate::common::patch_lifetime;
use crate::generation::common::*;
use crate::generation::fn_info::*;
use crate::generation::targets::common::*;
use crate::generation::targets::models::*;
use not_enough_syntax::*;
use quote::format_ident;
use syn::spanned::Spanned;
use syn::*;

pub(crate) fn generate_module(mut item_struct: ItemStruct) -> MockMod {
    item_struct = source_struct_fields::modify(item_struct);
    let source_span = item_struct.span();
    let generics_for_control_structs =
        patch_lifetime::prepend_to_generics(item_struct.generics.clone());
    let generics_for_impl = generics::remove_defaults(generics_for_control_structs.clone());
    let struct_setup_struct = struct_setup_struct::generate(
        source_span,
        struct_setup_struct::Params {
            struct_ident: &item_struct.ident,
            generics: generics_for_control_structs.clone(),
            is_static: false,
        },
    );
    let struct_setup_struct_clone_impl = clone_impl::generate(
        source_span,
        generics_for_impl.clone(),
        path::from_ident_with_generics(struct_setup_struct.ident.clone(), &generics_for_impl),
        &struct_setup_struct.fields,
    );
    let struct_static_setup_struct = struct_setup_struct::generate(
        source_span,
        struct_setup_struct::Params {
            struct_ident: &item_struct.ident,
            generics: generics_for_control_structs.clone(),
            is_static: true,
        },
    );
    let struct_static_setup_struct_clone_impl = clone_impl::generate(
        source_span,
        generics_for_impl.clone(),
        path::from_ident_with_generics(
            struct_static_setup_struct.ident.clone(),
            &generics_for_impl,
        ),
        &struct_static_setup_struct.fields,
    );
    let mockable_trait_impl = mockable_trait_impl::generate(
        source_span,
        mockable_trait_impl::Params {
            struct_ident: item_struct.ident.clone(),
            generics: generics_for_impl,
            setup_struct_ident: struct_setup_struct.ident.clone(),
            static_setup_struct_ident: struct_static_setup_struct.ident.clone(),
        },
    );
    let generics_mock_generics_info_provider =
        generics::remove_defaults(item_struct.generics.clone());
    let mock_generics_info_provider_impl = generics_info_provider_impl::generate_mock_impl(
        source_span,
        generics_mock_generics_info_provider.clone(),
        Type::Path(TypePath {
            attrs: Vec::new(),
            qself: None,
            path: path::from_ident_with_generics(
                item_struct.ident.clone(),
                &generics_mock_generics_info_provider,
            ),
        }),
    );

    let mod_visibility = item_struct.vis.clone();
    let mod_ident = format_ident!("__automock_generated_{}Mock", item_struct.ident);
    let use_super = use_super::new(source_span);
    let items = vec![
        Item::Use(use_super),
        Item::Impl(mockable_trait_impl),
        Item::Impl(mock_generics_info_provider_impl),
        Item::Struct(struct_setup_struct),
        Item::Impl(struct_setup_struct_clone_impl),
        Item::Struct(struct_static_setup_struct),
        Item::Impl(struct_static_setup_struct_clone_impl),
    ];
    let item_mod = ItemMod {
        attrs: vec![
            attributes::allow_clippy(source_span, "all"),
            attributes::allow_unused(source_span),
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
        source_item: Item::Struct(item_struct),
        maybe_usage: None,
        item_mod,
    };
    return result;
}
