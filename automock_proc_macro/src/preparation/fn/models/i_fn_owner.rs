use syn::*;

pub(crate) trait IFnOwner {
    fn maybe_ident(&self) -> Option<&Ident>;
    fn name(&self) -> String;
    fn maybe_trait_path(&self) -> Option<Path> {
        None
    }
    fn generics(&self) -> &Generics;
}
