use crate::generation::mock_setup::models::*;
use syn::*;

pub(crate) struct TraitStaticControls {
    pub trait_static_setup_struct: StaticSetupStruct,
    pub static_setup_struct_impl: ItemImpl,
}
