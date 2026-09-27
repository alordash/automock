use crate::preparation::models::*;
use crate::preparation::r#fn::models::*;
use syn::*;

pub(crate) struct ImplTraitForStructSyntax {
    pub attributes: Vec<Attribute>,
    pub target_path: Path,
    pub trait_ident: Ident,
    pub trait_path: Path,
    pub merged_generics: Generics,
    pub target_simple_generics: Generics,
    pub as_trait_where_predicates: Vec<WherePredicate>,
    pub trait_simple_generics: Generics,
    pub constants: Vec<Ordered<ImplItemConst>>,
    pub types: Vec<Ordered<ImplItemType>>,
    pub static_fns: Vec<Ordered<FnSyntax>>,
    pub associated_fns: Vec<Ordered<FnSyntax>>,
}
