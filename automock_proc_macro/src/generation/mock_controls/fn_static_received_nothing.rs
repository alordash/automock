use crate::common::*;
use crate::generation::fn_info::models::*;
use not_enough_syntax::*;
use proc_macro2::Span;
use syn::punctuated::Punctuated;
use syn::*;

pub(crate) fn generate(span: Span, mock_struct_path: Path, fn_info: &FnInfo) -> ItemFn {
    let automock_lifetime = patch_lifetime::new(span);
    let sig = Signature {
        constness: None,
        asyncness: None,
        safety: Safety::Default,
        abi: None,
        fn_token: Token![fn](span),
        ident: Ident::new("received_nothing", span),
        generics: generics::with_lifetimes_tied_to(
            fn_info.merged_generics.clone(),
            &fn_info.merged_generics,
            automock_lifetime.clone(),
        ),
        paren_token: token::Paren(span),
        inputs: Punctuated::new(),
        variadic: None,
        output: ReturnType::Default,
    };
    let verify_static_fn_received_nothing_else_stmt = Expr::Call(expr::call::new(
        span,
        Expr::Path(ExprPath {
            attrs: Vec::new(),
            qself: None,
            path: path::new_generics_global(
                span,
                for_generated::new("verify_static_fn_received_nothing_else"),
                [GenericArgument::Type(Type::Path(TypePath {
                    attrs: Vec::new(),
                    qself: None,
                    path: path::remove_lifetime_generic_arguments(mock_struct_path),
                }))],
            ),
        }),
        [],
    ));

    let block = Block {
        brace_token: token::Brace(span),
        stmts: vec![Stmt::Expr(
            verify_static_fn_received_nothing_else_stmt,
            None,
        )],
    };

    let result = ItemFn {
        attrs: Vec::new(),
        vis: Visibility::Public(Token![pub](span)),
        modifiers: FnModifiers::default(),
        sig,
        block: Box::new(block),
    };
    return result;
}
