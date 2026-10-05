use crate::generation::targets;
use crate::generation::targets::models::MockMod;
use quote::quote;
use syn::{Item, parse_macro_input};

mod common;
mod constants;
mod generation;
mod preparation;

/// The whole point. Apply it to function/trait/struct/`impl` block to turn it into mockable object.
#[proc_macro_attribute]
pub fn mock(
    _: proc_macro::TokenStream,
    proc_macro_item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = parse_macro_input!(proc_macro_item as Item);
    let mock_mod = match item {
        Item::Fn(item_fn) => targets::r#fn::generate_module(item_fn),
        Item::Impl(item_impl) => {
            if item_impl.trait_.is_some() {
                targets::impl_trait_for_struct::generate_module(item_impl)
            } else {
                targets::impl_struct::generate_module(item_impl)
            }
        }
        Item::Struct(item_struct) => targets::r#struct::generate_module(item_struct),
        Item::Trait(item_trait) => targets::r#trait::generate_module(item_trait),
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
