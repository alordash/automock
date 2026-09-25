use crate::common::*;
use not_enough_syntax::*;
use proc_macro2::Span;
use syn::*;

pub(crate) fn new(span: Span) -> (ExprPath, PatType) {
    let times_arg_path = expr::path::new(span, ["times"]);
    let times_arg = PatType {
        attrs: Vec::new(),
        pat: Box::new(Pat::Path(times_arg_path.clone())),
        colon_token: Token![:](span),
        ty: Box::new(Type::Path(r#type::path::new_global(
            span,
            for_generated::new("Times"),
        ))),
    };
    return (times_arg_path, times_arg);
}
