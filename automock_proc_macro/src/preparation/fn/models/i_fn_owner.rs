use syn::*;

pub(crate) trait IFnOwner {
    fn maybe_ident(&self) -> Option<&Ident>;
    fn name(&self) -> String;
    fn maybe_trait_name(&self) -> Option<String> {
        None
    }
    fn generics(&self) -> &Generics;
}
