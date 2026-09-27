use crate::preparation::models::*;
use crate::preparation::r#fn::models::*;
use crate::preparation::r#trait::models::*;
use syn::*;

pub(crate) struct TraitSyntax {
    pub unsafety: Option<Token![unsafe]>,
    pub ident: Ident,
    pub merged_generics: Generics,
    pub constants: Vec<Ordered<TraitItemConstSyntax>>,
    pub assoc_types: Vec<Ordered<TraitItemTypeSyntax>>,
    pub path: Path,
    pub static_fns: Vec<Ordered<FnSyntax>>,
    pub associated_fns: Vec<Ordered<FnSyntax>>,
}
