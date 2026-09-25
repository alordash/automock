use crate::common::generics_phantom_data;
use not_enough_syntax::*;
use proc_macro2::Span;
use syn::*;

pub(crate) fn new_field(
    span: Span,
    generics: &Generics,
    maybe_argument_types: Option<Vec<Type>>,
) -> Field {
    let result = Field {
        attrs: Vec::new(),
        vis: Visibility::Public(Token![pub](span)),
        modifiers: FieldModifiers::default(),
        ident: Some(generics_field_ident(span)),
        colon_token: Some(Token![:](span)),
        ty: generics_phantom_data::new(
            span,
            generics_phantom_data::Params {
                generics,
                maybe_argument_types,
            },
        ),
        default: None,
    };
    return result;
}

pub(crate) fn new_value(span: Span) -> FieldValue {
    let result = FieldValue {
        attrs: Vec::new(),
        member: Member::Named(generics_field_ident(span)),
        colon_token: Some(Token![:](span)),
        expr: Expr::Path(ExprPath {
            attrs: Vec::new(),
            qself: None,
            path: path::new_global(span, ["core", "marker", "PhantomData"]),
        }),
    };
    return result;
}
