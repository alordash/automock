use std::collections::HashSet;
use syn::*;

pub(crate) struct AssociatedItemsInfo {
    pub trait_path_segment: PathSegment,
    pub associated_items_ident_strings: HashSet<String>,
}
