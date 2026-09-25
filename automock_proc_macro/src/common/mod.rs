pub(crate) mod models {
    mod associated_items_info;
    mod context;

    pub(crate) use associated_items_info::*;
    pub(crate) use context::*;
}

pub(crate) mod context;
pub(crate) mod data_field;
pub(crate) mod generics_field;
pub(crate) mod generics_phantom_data;
pub(crate) mod normalization;
pub(crate) mod for_generated;
pub(crate) mod patch_lifetime;
pub(crate) mod transmute_lifetime_expr;
