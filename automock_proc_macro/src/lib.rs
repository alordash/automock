use crate::common::context;
use crate::generation::targets;
use crate::generation::targets::models::MockMod;
use quote::quote;
use syn::{Item, parse_macro_input};

mod common;
mod constants;
mod generation;
mod preparation;

#[proc_macro_attribute]
pub fn mock(
    proc_macro_attribute: proc_macro::TokenStream,
    proc_macro_item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = parse_macro_input!(proc_macro_item as Item);
    let mock_mod = match item {
        Item::Fn(item_fn) => {
            let ctx = context::create_for_mock_macro(proc_macro_attribute);
            targets::r#fn::generate_module(&ctx, item_fn)
        }
        Item::Impl(item_impl) => {
            if item_impl.trait_.is_some() {
                let ctx = context::create_for_mock_macro(proc_macro_attribute);
                targets::impl_trait_for_struct::generate_module(&ctx, item_impl)
            } else {
                let ctx = context::create_for_mock_macro(proc_macro_attribute);
                targets::impl_struct::generate_module(&ctx, item_impl)
            }
        }
        Item::Struct(item_struct) => targets::r#struct::generate_module(item_struct),
        Item::Trait(item_trait) => {
            let ctx = context::create_for_mock_macro(proc_macro_attribute);
            targets::r#trait::generate_module(&ctx, item_trait)
        }
        _ => panic!("Can mock only `fn`, `trait`, `struct` or `impl`."),
    };

    let MockMod {
        source_item,
        maybe_usage,
        item_mod,
    } = mock_mod;
    let result = quote! {
        #source_item
        #maybe_usage
        #item_mod
    };
    return result.into();
}
