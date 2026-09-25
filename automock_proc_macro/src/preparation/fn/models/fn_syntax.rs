use super::*;
use proc_macro2::Span;
use syn::*;

pub(crate) struct FnSyntax {
    pub spans: Spans,
    pub attributes: Vec<Attribute>,
    pub source_signature: Signature,
    pub signature: Box<Signature>,
    pub visibility: Visibility,
    pub merged_generics: Generics,
    pub generics_field: Field,
    pub maybe_owner_name: Option<String>,
    pub fn_ident: Ident,
    pub fn_data_name: String,
    pub maybe_self_type: Option<Receiver>,
    pub arguments: Vec<Argument>,
    pub arg_refs_tuple: TypeTuple,
    pub maybe_base_impl: Option<Box<Block>>,
    pub return_type: ReturnType,
}

pub(crate) struct Spans {
    pub inputs: Span,
}
