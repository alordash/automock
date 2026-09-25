mod assoc_items_normalization;
mod impl_trait_normalization;
mod struct_type_references_normalization;
mod super_path_adjusting;

pub(crate) use assoc_items_normalization::*;
pub(crate) use impl_trait_normalization::*;
pub(crate) use struct_type_references_normalization::*;
pub(crate) use super_path_adjusting::*;
