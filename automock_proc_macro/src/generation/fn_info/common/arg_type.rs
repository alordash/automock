use crate::common::*;
use not_enough_syntax::*;
use proc_macro2::Span;
use syn::*;

pub(crate) fn of(span: Span, r#type: Type) -> TypePath {
    let result = TypePath {
        attrs: Vec::new(),
        qself: None,
        path: path::new_generics_global(
            span,
            for_generated::new("Arg"),
            [GenericArgument::Type(r#type)],
        ),
    };

    return result;
}
