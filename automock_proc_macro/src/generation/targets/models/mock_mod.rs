use syn::*;

pub(crate) struct MockMod {
    pub source_item: Item,
    pub maybe_usage: Option<ItemUse>,
    pub item_mod: ItemMod,
}
