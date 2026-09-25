use crate::common::*;
use not_enough_syntax::*;
use proc_macro2::Span;
use syn::*;

pub(crate) fn new_field(span: Span) -> Field {
    let result = Field {
        attrs: vec![attributes::doc_hidden(span)],
        vis: Visibility::Public(Token![pub](span)),
        modifiers: FieldModifiers::default(),
        ident: Some(Ident::new("__am_data", span)),
        colon_token: Some(Token![:](span)),
        ty: Type::Path(TypePath {
            attrs: Vec::new(),
            qself: None,
            path: Path {
                leading_colon: Some(Token![::](span)),
                segments: for_generated::punctuated(
                    span,
                    [PathSegment {
                        ident: Ident::new("SharedMockData", span),
                        arguments: PathArguments::None,
                    }],
                ),
            },
        }),
        default: None,
    };
    return result;
}

pub(crate) fn new_default_value(span: Span) -> FieldValue {
    let result = FieldValue {
        attrs: Vec::new(),
        member: Member::Named(Ident::new("__am_data", span)),
        colon_token: Some(Token![:](span)),
        expr: Expr::Call(expr::call::new(
            span,
            Expr::Path(ExprPath {
                attrs: Vec::new(),
                qself: None,
                path: path::new_global(span, ["core", "default", "Default", "default"]),
            }),
            [],
        )),
    };
    return result;
}

pub(crate) fn new_clone_value(span: Span) -> FieldValue {
    let result = FieldValue {
        attrs: Vec::new(),
        member: Member::Named(Ident::new("__am_data", span)),
        colon_token: Some(Token![:](span)),
        expr: Expr::MethodCall(expr::method_call::new(
            span,
            Expr::Field(expr::field::new(
                Expr::Path(self_expr_path(span)),
                Ident::new("__am_data", span),
            )),
            Ident::new("clone", span),
            [],
        )),
    };
    return result;
}
