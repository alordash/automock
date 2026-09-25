use syn::*;

#[derive(Clone)]
pub(crate) struct TraitItemConstSyntax {
    pub corresponding_generic_param_path: Path,
    pub item: TraitItemConst,
}
