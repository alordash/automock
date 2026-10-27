use crate::common::*;
use crate::generation::common::*;
use crate::generation::fn_info::models::*;
use not_enough_syntax::*;
use proc_macro2::Span;
use quote::ToTokens;
use syn::punctuated::Punctuated;
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
    let maybe_trait_name = match &fn_info.maybe_trait_path {
        Some(trait_path) => Expr::Call(expr::call::new(
            span,
            Expr::Path(expr::path::new(span, ["Some"])),
            [trait_name_expr(span, trait_path, fn_info)],
        )),
        None => Expr::Path(expr::path::new(span, ["None"])),
    };
    let fn_data_stmt = Local {
        attrs: Vec::new(),
        let_token: Token![let](span),
        modifiers: LocalModifiers::default(),
        pat: fn_data_pat(span, fn_data_var_path.clone(), generic_arguments),
        init: Some(LocalInit {
            eq_token: Token![=](span),
            expr: Box::new(Expr::MethodCall(expr::method_call::new(
                span,
                Expr::Field(data_field::new_expr(span)),
                Ident::new("get_fn_data", span),
                [
                    expr::lit::string(span, owner_name),
                    maybe_trait_name,
                    expr::lit::string(span, &fn_info.fn_data_name),
                    Expr::Call(expr::call::new(
                        span,
                        Expr::Path(expr::path::new_global(
                            span,
                            for_generated::new2("IGenericsInfoProvider", "get_generics_hash_key"),
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
        Some(owner_name) => {
            let owner_type_name = match &fn_info.maybe_trait_path {
                Some(trait_path) => {
                    let trait_name = ident::combine_path_segments(trait_path);
                    format!("<{owner_name} as {trait_name}>")
                }
                None => owner_name.clone(),
            };
            (
                "get_static_fn_data_for_struct",
                punctuated([expr::lit::string(span, &owner_type_name), fn_name]),
            )
        }
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

fn trait_name_expr(span: Span, trait_path: &Path, fn_info: &FnInfo) -> Expr {
    let trait_name = ident::combine_path_segments(trait_path).to_string();
    let arguments = match &trait_path
        .segments
        .last()
        .expect("Trait path must contain at least one segment.")
        .arguments
    {
        PathArguments::AngleBracketed(arguments) => arguments
            .args
            .iter()
            .filter_map(|argument| match argument {
                GenericArgument::Type(ty) => {
                    if let Type::Path(type_path) = ty
                        && fn_info.merged_generics.const_params().any(|param| {
                            type_path.qself.is_none() && type_path.path.is_ident(&param.ident)
                        })
                    {
                        return Some(Expr::Path(ExprPath {
                            attrs: Vec::new(),
                            qself: None,
                            path: type_path.path.clone(),
                        }));
                    }
                    Some(Expr::Call(expr::call::new(
                        span,
                        Expr::Path(expr::path::new_generics(
                            span,
                            ["core", "any", "type_name"],
                            GenericArgument::Type(ty.clone()),
                        )),
                        [],
                    )))
                }
                GenericArgument::Const(value) => Some(value.clone()),
                _ => None,
            })
            .collect::<Vec<_>>(),
        _ => Vec::new(),
    };
    if arguments.is_empty() {
        return Expr::MethodCall(expr::method_call::new(
            span,
            expr::lit::string(span, &trait_name),
            Ident::new("to_owned", span),
            [],
        ));
    }
    let format_string = format!("{trait_name}<{}>", vec!["{}"; arguments.len()].join(", "));
    let format_arguments: Punctuated<_, Token![,]> =
        std::iter::once(expr::lit::string(span, &format_string))
            .chain(arguments)
            .collect();
    let result = Expr::Macro(ExprMacro {
        attrs: Vec::new(),
        mac: Macro {
            path: path::new(span, ["format"]),
            bang_token: Token![!](span),
            delimiter: MacroDelimiter::Paren(token::Paren(span)),
            tokens: format_arguments.to_token_stream(),
        },
    });
    return result;
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
