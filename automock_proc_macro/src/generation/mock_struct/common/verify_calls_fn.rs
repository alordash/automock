use crate::common::*;
use not_enough_syntax::*;
use proc_macro2::Span;
use syn::*;

pub(crate) struct Params {
    pub for_mockable_trait: bool,
}

pub(crate) fn generate(span: Span, Params { for_mockable_trait }: Params) -> ImplItemFn {
    let sig = Signature {
        constness: None,
        asyncness: None,
        safety: Safety::Default,
        abi: None,
        fn_token: Token![fn](span),
        ident: Ident::new("verify_calls", span),
        generics: Generics::default(),
        paren_token: token::Paren(span),
        inputs: punctuated([mut_ref_self_fn_arg(span)]),
        variadic: None,
        output: ReturnType::Default,
    };

    let verify_calls_stmt = Expr::MethodCall(expr::method_call::new(
        span,
        Expr::Field(data_field::new_expr(span)),
        Ident::new("verify_calls", span),
        [],
    ));
    let block = Block {
        brace_token: token::Brace(span),
        stmts: vec![Stmt::Expr(verify_calls_stmt, None)],
    };

    let result = ImplItemFn {
        attrs: Vec::new(),
        vis: if for_mockable_trait {
            Visibility::Inherited
        } else {
            Visibility::Public(Token![pub](span))
        },
        modifiers: FnModifiers::default(),
        sig,
        block,
    };

    return result;
}
