use syn::*;

pub(crate) struct StaticReceivedStruct {
    pub path: Path,
    pub item_struct: ItemStruct,
    pub clone_impl: ItemImpl,
    pub item_impl: ItemImpl,
}
