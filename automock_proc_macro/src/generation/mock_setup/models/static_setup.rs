use syn::*;

pub(crate) struct StaticSetupStruct {
    pub path: Path,
    pub item_struct: ItemStruct,
    pub item_impl: ItemImpl,
}
