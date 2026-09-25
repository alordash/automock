use syn::*;

pub(crate) struct ArgsCheckerStruct {
    pub path: Path,
    pub item_struct: ItemStruct,
    pub generics_info_provider_impl: ItemImpl,
    pub args_checker_impl: ItemImpl,
}
