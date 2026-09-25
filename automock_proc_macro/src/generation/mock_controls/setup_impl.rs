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
    pub setup_struct_path: Path,
    pub generics: Generics,
    pub mock_struct_path: &'a Path,
    pub fn_infos: &'a [T],
    pub for_static_fn: bool,
    pub is_static: bool,
}

pub(crate) fn generate<T: Borrow<FnInfo>>(
    ctx: &Context,
    span: Span,
    Params {
        setup_struct_path,
        generics,
        mock_struct_path,
        fn_infos,
        for_static_fn,
        is_static,
    }: Params<T>,
) -> ItemImpl {
    let items = fn_infos
        .iter()
        .map(|fn_info| {
            generate_setup_fn(
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
            path: setup_struct_path,
        })),
        brace_token: token::Brace(span),
        items,
    };
    return result;
}

fn generate_setup_fn(
    ctx: &Context,
    span: Span,
    mock_struct_path: &Path,
    fn_info: &FnInfo,
    for_static_fn: bool,
    is_static: bool,
) -> ImplItemFn {
    let generic_arguments = generic_arguments::new(
        ctx,
        span,
        generic_arguments::Params {
            mock_struct_path: mock_struct_path.clone(),
            fn_info,
            remove_lifetime_generic_arguments: false,
        },
    );
    let fn_configurator_path = fn_configurator_path::new(
        span,
        mock_struct_path.clone(),
        fn_info,
        &generic_arguments,
        placeholder_lifetime(span),
        None,
    );
    let mut generics = Generics::default();
    if !for_static_fn {
        generics = generics::combine(generics, &fn_info.signature.generics);
    }
    let sig = Signature {
        constness: None,
        asyncness: None,
        safety: Safety::Default,
        abi: None,
        fn_token: Token![fn](span),
        ident: if for_static_fn {
            Ident::new("setup", span)
        } else {
            fn_info.signature.ident.clone()
        },
        generics,
        paren_token: token::Paren(span),
        inputs: [ref_self_fn_arg(span)]
            .into_iter()
            .chain(fn_info.arguments.iter().map(|x| x.control_fn_arg.clone()))
            .collect(),
        variadic: None,
        output: ReturnType::Type(
            Token![->](span),
            Box::new(Type::Path(TypePath {
                attrs: Vec::new(),
                qself: None,
                path: fn_configurator_path.clone(),
            })),
        ),
    };

    let mut fn_configurator_path_for_var = fn_configurator_path;
    let Some(PathArguments::AngleBracketed(fn_configurator_path_for_var_args)) =
        fn_configurator_path_for_var
            .segments
            .last_mut()
            .map(|x| &mut x.arguments)
    else {
        panic!("FnConfigurator should have angle bracketed arguments.");
    };
    fn_configurator_path_for_var_args.args[0] =
        GenericArgument::Lifetime(placeholder_lifetime(span));
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
    let fn_configurator_var_path = path::new(span, ["fn_configurator"]);
    let fn_configurator_stmt = Local {
        attrs: Vec::new(),
        let_token: Token![let](span),
        modifiers: LocalModifiers::default(),
        pat: Pat::Type(PatType {
            attrs: Vec::new(),
            pat: Box::new(Pat::Path(ExprPath {
                attrs: Vec::new(),
                qself: None,
                path: fn_configurator_var_path.clone(),
            })),
            colon_token: Token![:](span),
            ty: Box::new(Type::Path(TypePath {
                attrs: Vec::new(),
                qself: None,
                path: fn_configurator_path_for_var,
            })),
        }),
        init: Some(LocalInit {
            eq_token: Token![=](span),
            expr: Box::new(Expr::MethodCall(expr::method_call::new(
                span,
                Expr::Path(fn_data_var_path.clone()),
                Ident::new("add_config", span),
                [
                    Expr::Path(args_checker_var_path),
                    Expr::Path(self_expr_path(span)),
                ],
            ))),
            diverge: None,
        }),
        semi_token: Token![;](span),
    };
    let return_stmt = Expr::Macro(transmute_lifetime_expr::new(Expr::Path(ExprPath {
        attrs: Vec::new(),
        qself: None,
        path: fn_configurator_var_path,
    })));

    let block = Block {
        brace_token: token::Brace(span),
        stmts: vec![
            Stmt::Local(args_checker_stmt),
            Stmt::Local(fn_data_stmt),
            Stmt::Local(fn_configurator_stmt),
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
