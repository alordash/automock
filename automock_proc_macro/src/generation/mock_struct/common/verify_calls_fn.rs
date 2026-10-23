use crate::common::*;
use not_enough_syntax::*;
use proc_macro2::Span;
use syn::punctuated::Punctuated;
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
        [get_generic_parameter_infos_call(span)],
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

pub(crate) struct StaticParams {
    pub mock_struct_path: Path,
    pub for_mockable_trait: bool,
}

pub(crate) fn generate_static(
    span: Span,
    StaticParams {
        mock_struct_path,
        for_mockable_trait,
    }: StaticParams,
) -> ImplItemFn {
    let sig = Signature {
        constness: None,
        asyncness: None,
        safety: Safety::Default,
        abi: None,
        fn_token: Token![fn](span),
        ident: Ident::new("verify_static_calls", span),
        generics: Generics::default(),
        paren_token: token::Paren(span),
        inputs: Punctuated::new(),
        variadic: None,
        output: ReturnType::Default,
    };

    let verify_static_fn_calls_stmt = Expr::Call(expr::call::new(
        span,
        Expr::Path(ExprPath {
            attrs: Vec::new(),
            qself: None,
            path: path::new_generics_global(
                span,
                ["automock", "for_generated", "verify_static_fn_calls"],
                [GenericArgument::Type(Type::Path(TypePath {
                    attrs: Vec::new(),
                    qself: None,
                    path: mock_struct_path,
                }))],
            ),
        }),
        [get_generic_parameter_infos_call(span)],
    ));

    let block = Block {
        brace_token: token::Brace(span),
        stmts: vec![Stmt::Expr(verify_static_fn_calls_stmt, None)],
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

fn get_generic_parameter_infos_call(span: Span) -> Expr {
    let trait_path_length = for_generated::new("IMockGenericsInfoProvider").len();
    let fn_path = ExprPath {
        attrs: Vec::new(),
        qself: Some(QSelf {
            lt_token: Token![<](span),
            ty: Box::new(Type::Path(r#type::path::new(span, ["Self"]))),
            position: trait_path_length,
            as_token: Some(Token![as](span)),
            gt_token: Token![>](span),
        }),
        path: path::new_global(
            span,
            for_generated::new2("IMockGenericsInfoProvider", "get_generic_parameter_infos"),
        ),
    };
    let result = Expr::Call(expr::call::new(span, Expr::Path(fn_path), []));
    return result;
}
