use crate::common::models::*;
use crate::common::*;
use crate::generation::common::*;
use crate::generation::fn_info::models::*;
use crate::generation::mock_controls::common::*;
use not_enough_syntax::*;
use proc_macro2::Span;
use syn::*;

pub(crate) fn generate(
    ctx: &Context,
    span: Span,
    mock_path: Path,
    static_setup_path: Path,
    fn_info: &FnInfo,
) -> ItemFn {
    let generic_arguments = generic_arguments::new(
        ctx,
        span,
        generic_arguments::Params {
            mock_struct_path: mock_path.clone(),
            fn_info,
            remove_lifetime_generic_arguments: false,
        },
    );
    let fn_configurator_path = fn_configurator_path::new(
        span,
        mock_path,
        fn_info,
        &generic_arguments,
        patch_lifetime::new(span),
        Some(Type::Path(TypePath {
            attrs: Vec::new(),
            qself: None,
            path: static_setup_path.clone(),
        })),
    );

    let sig = Signature {
        constness: None,
        asyncness: None,
        safety: Safety::Default,
        abi: None,
        fn_token: Token![fn](span),
        ident: Ident::new("setup", span),
        generics: fn_info.merged_generics.clone(),
        paren_token: token::Paren(span),
        inputs: fn_info
            .arguments
            .iter()
            .map(|x| x.control_fn_arg.clone())
            .collect(),
        variadic: None,
        output: ReturnType::Type(
            Token![->](span),
            Box::new(Type::Path(TypePath {
                attrs: Vec::new(),
                qself: None,
                path: fn_configurator_path,
            })),
        ),
    };

    let reset_fn_data_stmt = reset_fn_data_stmt::new(span, generic_arguments.mock_generic_argument);
    let setup_stmt = ExprMethodCall {
        attrs: Vec::new(),
        receiver: Box::new(Expr::Struct(ExprStruct {
            attrs: Vec::new(),
            qself: None,
            path: static_setup_path,
            brace_token: token::Brace(span),
            fields: punctuated([generics_field::new_value(span)]),
            dot2_token: None,
            rest: None,
        })),
        dot_token: Token![.](span),
        method: Ident::new("setup", span),
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
            .collect(),
    };

    let block = Block {
        brace_token: token::Brace(span),
        stmts: vec![
            Stmt::Expr(Expr::Call(reset_fn_data_stmt), Some(Token![;](span))),
            Stmt::Expr(Expr::MethodCall(setup_stmt), None),
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
