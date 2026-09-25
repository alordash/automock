use syn::*;

pub(crate) struct CallStruct {
    pub path: Path,
    pub item_struct: ItemStruct,
    pub generics_info_provider_impl: ItemImpl,
    pub call_impl: ItemImpl,
    pub maybe_clone_impl: Option<ItemImpl>,
}
