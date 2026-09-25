use crate::common::*;
use not_enough_syntax::*;
use proc_macro2::Span;
use syn::*;

pub(crate) fn new(span: Span, expr: Expr, target_type: Type) -> Expr {
    let arg_printer = Expr::Call(expr::call::new(
        span,
        Expr::Path(expr::path::new_global(
            span,
            for_generated::new("ArgPrinter"),
        )),
        [Expr::Macro(transmute_lifetime_expr::new_with_target(
            Expr::Reference(ExprReference {
                attrs: Vec::new(),
                and_token: Token![&](span),
                mutability: None,
                expr: Box::new(expr),
            }),
            Type::Reference(TypeReference {
                attrs: Vec::new(),
                and_token: Token![&](span),
                lifetime: None,
                mutability: None,
                elem: Box::new(target_type),
            }),
        ))],
    ));
    let arg_printer_ref = Expr::Reference(ExprReference {
        attrs: Vec::new(),
        and_token: Token![&](span),
        mutability: None,
        expr: Box::new(arg_printer),
    });
    let arg_printer_ref_paren = Expr::Paren(ExprParen {
        attrs: Vec::new(),
        paren_token: token::Paren(span),
        expr: Box::new(arg_printer_ref),
    });
    let result = expr::method_call::new(
        span,
        arg_printer_ref_paren,
        Ident::new("debug_string", span),
        [],
    );

    return Expr::MethodCall(result);
}
