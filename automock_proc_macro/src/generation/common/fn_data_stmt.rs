use crate::common::for_generated;
use crate::generation::common::*;
use crate::generation::fn_info::models::*;
use not_enough_syntax::*;
use proc_macro2::Span;
use syn::*;

pub(crate) struct AssociatedParams<'a> {
    pub fn_info: &'a FnInfo,
    pub generic_arguments: generic_arguments::Result,
    pub generics_info_provider_var_path: ExprPath,
}
pub(crate) fn new_associated(
    span: Span,
    AssociatedParams {
        fn_info,
        generic_arguments,
        generics_info_provider_var_path,
    }: AssociatedParams,
) -> (ExprPath, Local) {
    let fn_data_var_path = expr::path::new(span, ["fn_data"]);
    let owner_name = fn_info
        .maybe_owner_name
        .as_ref()
        .expect("Associated function must have owner name.");
    let fn_data_stmt = Local {
        attrs: Vec::new(),
        let_token: Token![let](span),
        modifiers: LocalModifiers::default(),
        pat: fn_data_pat(span, fn_data_var_path.clone(), generic_arguments),
        init: Some(LocalInit {
            eq_token: Token![=](span),
            expr: Box::new(Expr::Call(expr::call::new(
                span,
                Expr::Path(expr::path::new_global(
                    span,
                    for_generated::new2("ISharedMockData", "get_shared_fn_data"),
                )),
                [
                    Expr::Reference(ExprReference {
                        attrs: Vec::new(),
                        and_token: Token![&](span),
                        mutability: None,
                        expr: Box::new(Expr::Field(expr::field::new_self(Ident::new(
                            "__rs_data",
                            span,
                        )))),
                    }),
                    expr::lit::string(span, owner_name),
                    expr::lit::string(span, &fn_info.fn_data_name),
                    Expr::Call(expr::call::new(
                        span,
                        Expr::Path(expr::path::new_global(
                            span,
                            for_generated::new2(
                                "IGenericsInfoProvider",
                                "get_generics_hash_key",
                            ),
                        )),
                        [Expr::Reference(ExprReference {
                            attrs: Vec::new(),
                            and_token: Token![&](span),
                            mutability: None,
                            expr: Box::new(Expr::Path(generics_info_provider_var_path)),
                        })],
                    )),
                ],
            ))),
            diverge: None,
        }),
        semi_token: Token![;](span),
    };
    return (fn_data_var_path, fn_data_stmt);
}

pub(crate) struct StaticParams<'a> {
    pub fn_info: &'a FnInfo,
    pub generic_arguments: generic_arguments::Result,
}
pub(crate) fn new_static(
    span: Span,
    StaticParams {
        fn_info,
        generic_arguments,
    }: StaticParams,
) -> (ExprPath, Local) {
    let fn_data_var_path = expr::path::new(span, ["fn_data"]);
    let fn_name = expr::lit::string(span, &fn_info.fn_data_name);
    let (fn_data_fn_name, args) = match &fn_info.maybe_owner_name {
        None => ("get_static_fn_data", punctuated([fn_name])),
        Some(owner_name) => (
            "get_static_fn_data_for_struct",
            punctuated([expr::lit::string(span, owner_name), fn_name]),
        ),
    };
    let fn_data_stmt = Local {
        attrs: Vec::new(),
        let_token: Token![let](span),
        modifiers: LocalModifiers::default(),
        pat: fn_data_pat(span, fn_data_var_path.clone(), generic_arguments),
        init: Some(LocalInit {
            eq_token: Token![=](span),
            expr: Box::new(Expr::Call(ExprCall {
                attrs: Vec::new(),
                func: Box::new(Expr::Path(ExprPath {
                    attrs: Vec::new(),
                    qself: None,
                    path: path::new_global(span, for_generated::new(fn_data_fn_name)),
                })),
                paren_token: token::Paren(span),
                args,
            })),
            diverge: None,
        }),
        semi_token: Token![;](span),
    };
    return (fn_data_var_path, fn_data_stmt);
}

fn fn_data_pat(
    span: Span,
    data_var_path: ExprPath,
    generic_arguments: generic_arguments::Result,
) -> Pat {
    Pat::Type(PatType {
        attrs: Vec::new(),
        pat: Box::new(Pat::Path(data_var_path.clone())),
        colon_token: Token![:](span),
        ty: Box::new(Type::Reference(TypeReference {
            attrs: Vec::new(),
            and_token: Token![&](span),
            lifetime: None,
            mutability: None,
            elem: Box::new(Type::Path(TypePath {
                attrs: Vec::new(),
                qself: None,
                path: Path {
                    leading_colon: Some(Token![::](span)),
                    segments: punctuated([
                        PathSegment {
                            ident: Ident::new("automock", span),
                            arguments: PathArguments::None,
                        },
                        PathSegment {
                            ident: Ident::new("for_generated", span),
                            arguments: PathArguments::None,
                        },
                        PathSegment {
                            ident: Ident::new("FnData", span),
                            arguments: PathArguments::AngleBracketed(
                                AngleBracketedGenericArguments {
                                    colon2_token: None,
                                    lt_token: Token![<](span),
                                    args: punctuated([
                                        anonymous_lifetime_generic_argument(span),
                                        generic_arguments.mock_generic_argument,
                                        generic_arguments.has_return_value_argument,
                                        generic_arguments.supports_base_calling_argument,
                                        generic_arguments.passes_mock_to_callback_argument,
                                    ]),
                                    gt_token: Token![>](span),
                                },
                            ),
                        },
                    ]),
                },
            })),
        })),
    })
}
