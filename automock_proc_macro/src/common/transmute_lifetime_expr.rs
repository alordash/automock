use not_enough_syntax::*;
use quote::ToTokens;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::*;

pub(crate) fn new(expr: Expr) -> ExprMacro {
    let span = expr.span();
    let result = ExprMacro {
        attrs: Vec::new(),
        mac: Macro {
            path: path::new_global(span, ["automock", "transmute_lifetime"]),
            bang_token: Token![!](span),
            delimiter: MacroDelimiter::Paren(token::Paren(span)),
            tokens: expr.to_token_stream(),
        },
    };

    return result;
}

pub(crate) fn new_with_target(expr: Expr, target_type: Type) -> ExprMacro {
    let span = expr.span();
    let args: Punctuated<Expr, Token![,]> =
        punctuated([expr, Expr::Verbatim(target_type.to_token_stream())]);
    let result = ExprMacro {
        attrs: Vec::new(),
        mac: Macro {
            path: path::new_global(span, ["automock", "transmute_lifetime"]),
            bang_token: Token![!](span),
            delimiter: MacroDelimiter::Paren(token::Paren(span)),
            tokens: args.to_token_stream(),
        },
    };

    return result;
}
