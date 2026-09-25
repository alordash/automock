use crate::common::*;
use crate::generation::fn_info::models::*;
use crate::generation::mock_controls::common::*;
use not_enough_syntax::*;
use proc_macro2::Span;
use syn::*;

pub(crate) fn generate(span: Span, static_received_path: Path, fn_info: &FnInfo) -> ItemFn {
    let (times_arg_path, times_arg) = times_arg::new(span);
    let automock_lifetime = patch_lifetime::new(span);
    let output_type = Type::Path(TypePath {
        attrs: Vec::new(),
        qself: None,
        path: path::new_generics_global(
            span,
            for_generated::new("ArgRefsBinder"),
            [
                GenericArgument::Type(Type::Path(TypePath {
                    attrs: Vec::new(),
                    qself: None,
                    path: static_received_path.clone(),
                })),
                GenericArgument::Type(Type::Tuple(fn_info.arg_refs_tuple.clone())),
            ],
        ),
    });
    let sig = Signature {
        constness: None,
        asyncness: None,
        safety: Safety::Default,
        abi: None,
        fn_token: Token![fn](span),
        ident: Ident::new("received", span),
        generics: generics::with_lifetimes_tied_to(
            fn_info.merged_generics.clone(),
            &fn_info.merged_generics,
            automock_lifetime.clone(),
        ),
        paren_token: token::Paren(span),
        inputs: fn_info
            .arguments
            .iter()
            .map(|x| x.control_fn_arg.clone())
            .chain(core::iter::once(FnArg::Typed(times_arg)))
            .collect(),
        variadic: None,
        output: ReturnType::Type(Token![->](span), Box::new(output_type)),
    };

    let received_stmt = ExprMethodCall {
        attrs: Vec::new(),
        receiver: Box::new(Expr::Struct(ExprStruct {
            attrs: Vec::new(),
            qself: None,
            path: static_received_path,
            brace_token: token::Brace(span),
            fields: punctuated([generics_field::new_value(span)]),
            dot2_token: None,
            rest: None,
        })),
        dot_token: Token![.](span),
        method: Ident::new("received", span),
        turbofish: None,
        paren_token: token::Paren(span),
        args: fn_info
            .arguments
            .iter()
            .map(|x| {
                Expr::Path(ExprPath {
                    attrs: Vec::new(),
                    qself: None,
                    path: path::from_ident(x.ident.clone()),
                })
            })
            .chain(core::iter::once(Expr::Path(times_arg_path)))
            .collect(),
    };

    let block = Block {
        brace_token: token::Brace(span),
        stmts: vec![Stmt::Expr(Expr::MethodCall(received_stmt), None)],
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
