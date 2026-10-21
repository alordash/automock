use crate::generation::common::{fn_data_stmt, generic_arguments};
use crate::generation::fn_info::models::*;
use not_enough_syntax::expr;
use proc_macro2::Span;
use syn::punctuated::Punctuated;
use syn::*;

pub(crate) fn generate(span: Span, mock_path: Path, fn_info: &FnInfo) -> ItemFn {
    let sig = Signature {
        constness: None,
        asyncness: None,
        safety: Safety::Default,
        abi: None,
        fn_token: Token![fn](span),
        ident: Ident::new("verify_calls", span),
        generics: fn_info.merged_generics.clone(),
        paren_token: token::Paren(span),
        inputs: Punctuated::new(),
        variadic: None,
        output: ReturnType::Default,
    };

    let generic_arguments = generic_arguments::new(
        span,
        generic_arguments::Params {
            mock_struct_path: mock_path.clone(),
            fn_info,
            remove_lifetime_generic_arguments: false,
        },
    );
    let (fn_data_var_path, fn_data_stmt) = fn_data_stmt::new_static(
        span,
        fn_data_stmt::StaticParams {
            fn_info,
            generic_arguments,
        },
    );
    let validate_calls_stmt = Expr::MethodCall(expr::method_call::new(
        span,
        Expr::Path(fn_data_var_path),
        Ident::new("validate_calls", span),
        [],
    ));
    let block = Block {
        brace_token: token::Brace(span),
        stmts: vec![
            Stmt::Local(fn_data_stmt),
            Stmt::Expr(validate_calls_stmt, None),
        ],
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
