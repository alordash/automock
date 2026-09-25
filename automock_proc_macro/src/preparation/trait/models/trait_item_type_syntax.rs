use syn::*;

#[derive(Clone)]
pub(crate) struct TraitItemTypeSyntax {
    pub corresponding_generic_param_path: Path,
    pub item: TraitItemType,
}
