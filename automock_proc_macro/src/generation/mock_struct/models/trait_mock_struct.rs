use crate::generation::mock_struct::models::*;
use syn::*;

pub(crate) struct TraitMockStruct {
    pub item_struct: ItemStruct,
    pub clone_impl: ItemImpl,
    pub trait_impl: ItemImpl,
    pub inner_impl: ItemImpl,
    pub maybe_associated_controls: Option<AssociatedControls>,
    pub maybe_static_controls: Option<StaticControls>,
}
