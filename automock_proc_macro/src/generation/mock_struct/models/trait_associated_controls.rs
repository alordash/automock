use crate::generation::mock_controls::models::*;
use syn::*;

pub(crate) struct TraitAssociatedControls {
    pub trait_setup_struct: SetupStruct,
    pub trait_received_struct: ReceivedStruct,
    pub setup_struct_impl: ItemImpl,
    pub received_struct_impl: ItemImpl,
}
