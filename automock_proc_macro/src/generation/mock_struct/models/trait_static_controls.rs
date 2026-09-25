use crate::generation::mock_controls::models::*;
use syn::*;

pub(crate) struct TraitStaticControls {
    pub trait_static_setup_struct: StaticSetupStruct,
    pub trait_static_received_struct: StaticReceivedStruct,
    pub static_setup_struct_impl: ItemImpl,
    pub static_received_struct_impl: ItemImpl,
}
