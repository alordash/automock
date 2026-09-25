use crate::generation::fn_info::models::*;
use crate::preparation::models::*;
use proc_macro2::Ident;
use syn::*;

pub(crate) struct ImplStructInfo {
    pub attributes: Vec<Attribute>,
    pub target_ident: Ident,
    pub target_path: Path,
    pub generics: Generics,
    pub static_fns: Vec<Ordered<FnInfo>>,
    pub associated_fns: Vec<Ordered<FnInfo>>,
}
