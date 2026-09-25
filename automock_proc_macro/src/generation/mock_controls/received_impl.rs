use crate::common::models::*;
use crate::common::*;
use crate::generation::common::*;
use crate::generation::fn_info::models::*;
use crate::generation::mock_controls::common::*;
use not_enough_syntax::*;
use proc_macro2::{Ident, Span};
use std::borrow::Borrow;
use syn::*;

pub(crate) struct Params<'a, T: Borrow<FnInfo>> {
    pub received_struct_path: Path,
    pub generics: Generics,
    pub mock_struct_path: &'a Path,
    pub fn_infos: &'a [T],
    pub for_static_fn: bool,
    pub is_static: bool,
    pub generate_fn_no_other_calls: bool,
}
pub(crate) fn generate<T: Borrow<FnInfo>>(
    ctx: &Context,
    span: Span,
    Params {
        received_struct_path,
        generics,
        mock_struct_path,
        fn_infos,
        for_static_fn,
        is_static,
        generate_fn_no_other_calls,
    }: Params<T>,
) -> ItemImpl {
    let mut items: Vec<_> = fn_infos
        .iter()
        .map(|fn_info| {
            generate_received_fn(
                ctx,
                span,
                mock_struct_path,
                fn_info.borrow(),
                for_static_fn,
                is_static,
            )
        })
        .map(ImplItem::Fn)
        .collect();

    if generate_fn_no_other_calls {
        let fn_no_other_calls = if is_static {
            generate_fn_no_other_calls_for_static_fn(span, mock_struct_path.clone())
        } else {
            generate_regular_fn_no_other_calls(span)
        };
        items.push(ImplItem::Fn(fn_no_other_calls));
    }

    let result = ItemImpl {
        attrs: Vec::new(),
        modifiers: ImplModifiers::default(),
        unsafety: None,
        impl_token: Token![impl](span),
        generics,
        trait_: None,
        self_ty: Box::new(Type::Path(TypePath {
            attrs: Vec::new(),
            qself: None,
            path: received_struct_path,
        })),
        brace_token: token::Brace(span),
        items,
    };
    return result;
}

pub(crate) enum FnNoOtherCallsKind {
    Regular,
    ForStaticFn { mock_struct_path: Path },
}
pub(crate) struct ForStructParams {
    pub received_struct_path: Path,
    pub generics_for_impl: Generics,
    pub fn_no_other_calls_kind: FnNoOtherCallsKind,
}
pub(crate) fn generate_for_struct_with_fn_no_other_calls(
    span: Span,
    ForStructParams {
        received_struct_path,
        generics_for_impl,
        fn_no_other_calls_kind,
    }: ForStructParams,
) -> ItemImpl {
    let fn_no_other_calls = match fn_no_other_calls_kind {
        FnNoOtherCallsKind::Regular => generate_regular_fn_no_other_calls(span),
        FnNoOtherCallsKind::ForStaticFn { mock_struct_path } => {
            generate_fn_no_other_calls_for_static_fn(span, mock_struct_path)
        }
    };

    let result = ItemImpl {
        attrs: Vec::new(),
        modifiers: ImplModifiers::default(),
        unsafety: None,
        impl_token: Token![impl](span),
        generics: generics_for_impl,
        trait_: None,
        self_ty: Box::new(Type::Path(TypePath {
            attrs: Vec::new(),
            qself: None,
            path: received_struct_path,
        })),
        brace_token: token::Brace(span),
        items: vec![ImplItem::Fn(fn_no_other_calls)],
    };
    return result;
}

fn generate_received_fn(
    ctx: &Context,
    span: Span,
    mock_struct_path: &Path,
    fn_info: &FnInfo,
    for_static_fn: bool,
    is_static: bool,
) -> ImplItemFn {
    let (times_arg_path, times_arg) = times_arg::new(span);
    let generic_arguments = generic_arguments::new(
        ctx,
        span,
        generic_arguments::Params {
            mock_struct_path: mock_struct_path.clone(),
            fn_info,
            remove_lifetime_generic_arguments: true,
        },
    );
    let automock_lifetime = patch_lifetime::new(span);
    let mut generics = generics::with_lifetimes_tied_to(
        Generics::default(),
        &fn_info.merged_generics,
        automock_lifetime.clone(),
    );
    if !for_static_fn {
        generics = generics::combine(generics, &fn_info.signature.generics);
    }
    let self_arg = ref_self_fn_arg(span);
    let output_type = Type::Path(TypePath {
        attrs: Vec::new(),
        qself: None,
        path: path::new_generics_global(
            span,
            for_generated::new("ArgRefsBinder"),
            [
                GenericArgument::Type(Type::Path(self_type(span))),
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
        ident: if for_static_fn {
            Ident::new("received", span)
        } else {
            fn_info.signature.ident.clone()
        },
        generics,
        paren_token: token::Paren(span),
        inputs: [self_arg]
            .into_iter()
            .chain(fn_info.arguments.iter().map(|x| x.control_fn_arg.clone()))
            .chain(core::iter::once(FnArg::Typed(times_arg)))
            .collect(),
        variadic: None,
        output: ReturnType::Type(Token![->](span), Box::new(output_type)),
    };
    let (args_checker_var_path, args_checker_stmt) = args_checker_stmt::new(span, fn_info);
    let (fn_data_var_path, fn_data_stmt) = if is_static {
        fn_data_stmt::new_static(
            span,
            fn_data_stmt::StaticParams {
                fn_info,
                generic_arguments,
            },
        )
    } else {
        fn_data_stmt::new_associated(
            span,
            fn_data_stmt::AssociatedParams {
                fn_info,
                generic_arguments,
                generics_info_provider_var_path: args_checker_var_path.clone(),
            },
        )
    };
    let verify_received_stmt = Expr::MethodCall(expr::method_call::new(
        span,
        Expr::Path(fn_data_var_path),
        Ident::new("verify_received", span),
        [
            Expr::Path(args_checker_var_path),
            Expr::Path(times_arg_path),
        ],
    ));
    let return_stmt = Expr::Call(expr::call::new(
        span,
        Expr::Path(expr::path::new(
            span,
            for_generated::new2("ArgRefsBinder", "new"),
        )),
        [Expr::MethodCall(expr::method_call::new(
            span,
            Expr::Path(self_expr_path(span)),
            Ident::new("clone", span),
            [],
        ))],
    ));

    let block = Block {
        brace_token: token::Brace(span),
        stmts: vec![
            Stmt::Local(args_checker_stmt),
            Stmt::Local(fn_data_stmt),
            Stmt::Expr(verify_received_stmt, Some(Token![;](span))),
            Stmt::Expr(return_stmt, None),
        ],
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

fn generate_fn_no_other_calls_for_static_fn(span: Span, mock_struct_path: Path) -> ImplItemFn {
    let sig = fn_no_other_calls_signature(span);
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

    let result = ImplItemFn {
        attrs: Vec::new(),
        vis: Visibility::Public(Token![pub](span)),
        modifiers: FnModifiers::default(),
        sig,
        block,
    };
    return result;
}

fn generate_regular_fn_no_other_calls(span: Span) -> ImplItemFn {
    let sig = fn_no_other_calls_signature(span);
    let verify_received_nothing_else_stmt = expr::call::new(
        span,
        Expr::Path(expr::path::new(
            span,
            for_generated::new2("IMockData", "verify_received_nothing_else"),
        )),
        [Expr::Reference(ExprReference {
            attrs: Vec::new(),
            and_token: Token![&](span),
            mutability: None,
            expr: Box::new(Expr::Field(expr::field::new_self(Ident::new(
                "__rs_data",
                span,
            )))),
        })],
    );

    let block = Block {
        brace_token: token::Brace(span),
        stmts: vec![Stmt::Expr(
            Expr::Call(verify_received_nothing_else_stmt),
            None,
        )],
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

fn fn_no_other_calls_signature(span: Span) -> Signature {
    let result = Signature {
        constness: None,
        asyncness: None,
        safety: Safety::Default,
        abi: None,
        fn_token: Token![fn](span),
        ident: Ident::new("no_other_calls", span),
        generics: Generics::default(),
        paren_token: token::Paren(span),
        inputs: punctuated([ref_self_fn_arg(span)]),
        variadic: None,
        output: ReturnType::Default,
    };
    return result;
}
