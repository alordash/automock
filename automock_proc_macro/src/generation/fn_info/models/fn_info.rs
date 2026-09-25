use super::*;
use crate::preparation::r#fn::models::*;
use proc_macro2::Ident;
use syn::*;

pub(crate) struct FnInfo {
    pub spans: Spans,
    pub attributes: Vec<Attribute>,
    pub source_signature: Signature,
    pub signature: Box<Signature>,
    pub visibility: Visibility,
    pub merged_generics: Generics,
    pub maybe_owner_name: Option<String>,
    pub fn_ident: Ident,
    pub fn_data_name: String,
    pub maybe_self_type: Option<Receiver>,
    pub arguments: Vec<Argument>,
    pub arg_refs_tuple: TypeTuple,
    pub maybe_base_impl: Option<Box<Block>>,
    pub return_type: ReturnType,
    pub call_struct: CallStruct,
    pub args_checker_struct: ArgsCheckerStruct,
}
