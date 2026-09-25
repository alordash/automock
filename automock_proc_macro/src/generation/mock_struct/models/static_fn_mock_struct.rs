use syn::*;

pub(crate) struct StaticFnMockStruct {
    pub path: Path,
    pub item_struct: ItemStruct,
}
