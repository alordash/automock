use crate::common::*;
use crate::generation::common::*;
use crate::generation::mock_controls::models::*;
use not_enough_syntax::*;
use proc_macro2::Span;
use syn::punctuated::Punctuated;
use syn::*;

pub(crate) fn generate_associated(
    span: Span,
    control_path: Path,
    control_type: ControlType,
) -> ImplItemFn {
    let ident_str = match control_type {
        ControlType::Setup => "setup",
        ControlType::Received => "received",
    };

    let sig = Signature {
        constness: None,
        asyncness: None,
        safety: Safety::Default,
        abi: None,
        fn_token: Token![fn](span),
        ident: Ident::new(ident_str, span),
        generics: Generics::default(),
        paren_token: token::Paren(span),
        inputs: punctuated([mut_ref_self_fn_arg(span)]),
        variadic: None,
        output: ReturnType::Type(
            Token![->](span),
            Box::new(Type::Path(TypePath {
                attrs: Vec::new(),
                qself: None,
                path: control_path.clone(),
            })),
        ),
    };
    let constructor_stmt = Expr::Struct(ExprStruct {
        attrs: Vec::new(),
        qself: None,
        path: control_path,
        brace_token: token::Brace(span),
        fields: punctuated([
            generics_field::new_value(span),
            data_field::new_clone_value(span),
        ]),
        dot2_token: None,
        rest: None,
    });
    let block = Block {
        brace_token: token::Brace(span),
        stmts: vec![Stmt::Expr(constructor_stmt, None)],
    };

    let result = ImplItemFn {
        attrs: Vec::new(),
        vis: Visibility::Public(Token![pub](span)),
        modifiers: FnModifiers::default(),
        sig,
        block,
    };
    return result;
}

pub(crate) fn generate_static(
    span: Span,
    control_path: Path,
    static_control_type: StaticControlType,
) -> ImplItemFn {
    let ident_str = match static_control_type {
        StaticControlType::Setup { .. } => "static_setup",
        StaticControlType::Received => "static_received",
    };
    let sig = Signature {
        constness: None,
        asyncness: None,
        safety: Safety::Default,
        abi: None,
        fn_token: Token![fn](span),
        ident: Ident::new(ident_str, span),
        generics: Generics::default(),
        paren_token: token::Paren(span),
        inputs: Punctuated::new(),
        variadic: None,
        output: ReturnType::Type(
            Token![->](span),
            Box::new(Type::Path(TypePath {
                attrs: Vec::new(),
                qself: None,
                path: control_path.clone(),
            })),
        ),
    };
    let constructor_stmt = Expr::Struct(ExprStruct {
        attrs: Vec::new(),
        qself: None,
        path: control_path,
        brace_token: token::Brace(span),
        fields: punctuated([generics_field::new_value(span)]),
        dot2_token: None,
        rest: None,
    });
    let stmts = match static_control_type {
        StaticControlType::Setup {
            mock_generic_argument,
        } => {
            let reset_fn_data_stmt = reset_fn_data_stmt::new(span, *mock_generic_argument);
            vec![
                Stmt::Expr(Expr::Call(reset_fn_data_stmt), Some(Token![;](span))),
                Stmt::Expr(constructor_stmt, None),
            ]
        }
        StaticControlType::Received => vec![Stmt::Expr(constructor_stmt, None)],
    };
    let block = Block {
        brace_token: token::Brace(span),
        stmts,
    };

    let result = ImplItemFn {
        attrs: Vec::new(),
        vis: Visibility::Public(Token![pub](span)),
        modifiers: FnModifiers::default(),
        sig,
        block,
    };
    return result;
}
