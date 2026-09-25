use syn::*;

pub(crate) trait IFnOwner {
    fn maybe_ident(&self) -> Option<&Ident>;
    fn format_name(&self) -> String;
    fn generics(&self) -> &Generics;
}
