use crate::generation::fn_info::models::*;
use crate::preparation::models::*;
use proc_macro2::Ident;
use syn::*;

pub(crate) struct ImplTraitForStructInfo {
    pub attributes: Vec<Attribute>,
    pub target_path: Path,
    pub trait_ident: Ident,
    pub trait_path: Path,
    pub merged_generics: Generics,
    pub trait_simple_generics: Generics,
    pub as_trait_where_predicates: Vec<WherePredicate>,
    pub target_simple_generics: Generics,
    pub constants: Vec<Ordered<ImplItemConst>>,
    pub types: Vec<Ordered<ImplItemType>>,
    pub static_fns: Vec<Ordered<FnInfo>>,
    pub associated_fns: Vec<Ordered<FnInfo>>,
}
