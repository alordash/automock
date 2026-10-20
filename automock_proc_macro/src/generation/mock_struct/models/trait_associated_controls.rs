use crate::generation::mock_setup::models::*;
use syn::*;

pub(crate) struct TraitAssociatedControls {
    pub trait_setup_struct: SetupStruct,
    pub setup_struct_impl: ItemImpl,
}
