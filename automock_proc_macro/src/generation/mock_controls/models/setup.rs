use syn::*;

pub(crate) struct SetupStruct {
    pub path: Path,
    pub item_struct: ItemStruct,
    pub item_impl: ItemImpl,
}
