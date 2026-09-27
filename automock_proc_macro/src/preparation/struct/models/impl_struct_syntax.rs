use crate::preparation::models::*;
use crate::preparation::r#fn::models::*;
use syn::*;

pub(crate) struct ImplStructSyntax {
    pub attributes: Vec<Attribute>,
    pub target_ident: Ident,
    pub target_path: Path,
    pub generics: Generics,
    pub static_fns: Vec<Ordered<FnSyntax>>,
    pub associated_fns: Vec<Ordered<FnSyntax>>,
}
