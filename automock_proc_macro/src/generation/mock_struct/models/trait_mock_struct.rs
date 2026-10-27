use crate::generation::mock_controls::models::*;
use syn::*;

pub(crate) struct TraitMockStruct {
    pub item_struct: ItemStruct,
    pub trait_impl: ItemImpl,
    pub inner_impl: ItemImpl,
    pub mock_generics_info_provider_impl: ItemImpl,
    pub maybe_associated_setup: Option<SetupStruct>,
    pub maybe_static_setup: Option<StaticSetupStruct>,
}
