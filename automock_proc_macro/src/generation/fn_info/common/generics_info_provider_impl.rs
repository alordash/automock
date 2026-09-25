use crate::common::*;
use not_enough_syntax::*;
use proc_macro2::Span;
use quote::ToTokens;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::*;

pub(crate) fn generate(impl_generics: Generics, generics: Generics, target_type: Type) -> ItemImpl {
    let span = generics.span();
    let items = if generics.params.is_empty() {
        Vec::new()
    } else {
        let fn_get_generic_parameter_infos =
            generate_fn_get_generic_parameter_infos(span, generics.params.iter());
        let fn_hash_generics_type_ids =
            generate_fn_hash_generics_type_ids(span, generics.type_params());
        let fn_hash_const_values = generate_fn_hash_const_values(span, generics.const_params());
        vec![
            ImplItem::Fn(fn_get_generic_parameter_infos),
            ImplItem::Fn(fn_hash_generics_type_ids),
            ImplItem::Fn(fn_hash_const_values),
        ]
    };

    let result = ItemImpl {
        attrs: Vec::new(),
        modifiers: ImplModifiers::default(),
        unsafety: None,
        impl_token: Token![impl](span),
        generics: impl_generics,
        trait_: Some((
            path::new_global(
                span,
                for_generated::new("IGenericsInfoProvider"),
            ),
            Token![for](span),
        )),
        self_ty: Box::new(target_type),
        brace_token: token::Brace(span),
        items,
    };
    return result;
}

fn generate_fn_get_generic_parameter_infos<'a>(
    span: Span,
    generic_params: impl Iterator<Item = &'a GenericParam>,
) -> ImplItemFn {
    let sig = Signature {
        constness: None,
        asyncness: None,
        safety: Safety::Default,
        abi: None,
        fn_token: Token![fn](span),
        ident: Ident::new("get_generic_parameter_infos", span),
        generics: Generics::default(),
        paren_token: token::Paren(span),
        inputs: punctuated([ref_self_fn_arg(span)]),
        variadic: None,
        output: ReturnType::Type(
            Token![->](span),
            Box::new(Type::Path(r#type::vec_of(
                span,
                Type::Path(r#type::path::new_global(
                    span,
                    for_generated::new("GenericParameterInfo"),
                )),
            ))),
        ),
    };

    let generic_parameter_infos: Punctuated<Expr, Token![,]> = generic_params
        .filter_map(|generic_param| match generic_param {
            GenericParam::Type(type_param) => Some(Expr::Call(expr::call::new(
                span,
                Expr::Path(expr::path::new(
                    span,
                    for_generated::new("generic_type_info"),
                )),
                [
                    Expr::Lit(ExprLit {
                        attrs: Vec::new(),
                        lit: Lit::Str(LitStr::new(&type_param.ident.to_string(), span)),
                    }),
                    Expr::Call(expr::call::new(
                        span,
                        Expr::Path(expr::path::new_generics(
                            span,
                            ["core", "any", "type_name"],
                            GenericArgument::Type(Type::Path(TypePath {
                                attrs: Vec::new(),
                                qself: None,
                                path: path::from_ident(type_param.ident.clone()),
                            })),
                        )),
                        [],
                    )),
                ],
            ))),
            GenericParam::Const(const_param) => {
                let const_param_ident_string = const_param.ident.to_string();
                Some(Expr::Call(expr::call::new(
                    span,
                    Expr::Path(expr::path::new(
                        span,
                        for_generated::new("generic_const_info"),
                    )),
                    [
                        Expr::Lit(ExprLit {
                            attrs: Vec::new(),
                            lit: Lit::Str(LitStr::new(&const_param_ident_string, span)),
                        }),
                        Expr::Path(expr::path::new(span, [&const_param_ident_string])),
                    ],
                )))
            }
            _ => None,
        })
        .collect();

    let block = Block {
        brace_token: token::Brace(span),
        stmts: vec![Stmt::Macro(StmtMacro {
            attrs: Vec::new(),
            mac: r#macro::vec(span, generic_parameter_infos.to_token_stream()),
            semi_token: None,
        })],
    };

    let result = ImplItemFn {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        modifiers: FnModifiers::default(),
        sig,
        block,
    };
    return result;
}

fn generate_fn_hash_generics_type_ids<'a>(
    span: Span,
    type_params: impl Iterator<Item = &'a TypeParam>,
) -> ImplItemFn {
    let sig = generate_hash_fn_sig(span, "hash_generics_type_ids");
    let tids: Punctuated<_, _> = type_params
        .into_iter()
        .map(|type_param| {
            Expr::Call(expr::call::new(
                span,
                Expr::Path(expr::path::new_generics(
                    span,
                    for_generated::new("tid"),
                    GenericArgument::Type(Type::Path(TypePath {
                        attrs: Vec::new(),
                        qself: None,
                        path: Path {
                            leading_colon: None,
                            segments: punctuated([PathSegment {
                                ident: type_param.ident.clone(),
                                arguments: PathArguments::None,
                            }]),
                        },
                    })),
                )),
                [],
            ))
        })
        .collect();
    let stmts = if !tids.is_empty() {
        let tids_array = Expr::Array(ExprArray {
            attrs: Vec::new(),
            bracket_token: token::Bracket(span),
            elems: tids,
        });
        let hash_expr = expr::call::new(
            span,
            Expr::Path(expr::path::new_global(
                span,
                ["core", "hash", "Hash", "hash"],
            )),
            [
                Expr::Reference(ExprReference {
                    attrs: Vec::new(),
                    and_token: Token![&](span),
                    mutability: None,
                    expr: Box::new(tids_array),
                }),
                Expr::Path(expr::path::new(span, ["hasher"])),
            ],
        );
        let stmt = Stmt::Expr(Expr::Call(hash_expr), Some(Token![;](span)));
        vec![stmt]
    } else {
        Vec::new()
    };

    let result = ImplItemFn {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        modifiers: FnModifiers::default(),
        sig,
        block: Block {
            brace_token: token::Brace(span),
            stmts,
        },
    };
    return result;
}

fn generate_fn_hash_const_values<'a>(
    span: Span,
    const_params: impl Iterator<Item = &'a ConstParam>,
) -> ImplItemFn {
    let sig = generate_hash_fn_sig(span, "hash_const_values");
    let stmts = const_params
        .map(|const_param| {
            let const_hash_expr = Expr::Call(expr::call::new(
                span,
                Expr::Path(expr::path::new(
                    span,
                    for_generated::new("const_hash"),
                )),
                [
                    Expr::Reference(ExprReference {
                        attrs: Vec::new(),
                        and_token: Token![&](span),
                        mutability: None,
                        expr: Box::new(Expr::Path(ExprPath {
                            attrs: Vec::new(),
                            qself: None,
                            path: Path {
                                leading_colon: None,
                                segments: punctuated([PathSegment {
                                    ident: const_param.ident.clone(),
                                    arguments: PathArguments::None,
                                }]),
                            },
                        })),
                    }),
                    Expr::Path(expr::path::new(span, ["hasher"])),
                ],
            ));
            let stmt = Stmt::Expr(const_hash_expr, Some(Token![;](span)));
            return stmt;
        })
        .collect();

    let result = ImplItemFn {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        modifiers: FnModifiers::default(),
        sig,
        block: Block {
            brace_token: token::Brace(span),
            stmts,
        },
    };
    return result;
}

fn generate_hash_fn_sig(span: Span, fn_name: &'static str) -> Signature {
    let inputs = punctuated([
        ref_self_fn_arg(span),
        FnArg::Typed(PatType {
            attrs: Vec::new(),
            pat: Box::new(Pat::Ident(PatIdent {
                attrs: Vec::new(),
                by_ref: None,
                mutability: None,
                ident: Ident::new("hasher", span),
                subpat: None,
            })),
            colon_token: Token![:](span),
            ty: Box::new(Type::Reference(TypeReference {
                attrs: Vec::new(),
                and_token: Token![&](span),
                lifetime: None,
                mutability: Some(Token![mut](span)),
                elem: Box::new(Type::Path(r#type::path::new_global(
                    span,
                    for_generated::new("GenericsHasher"),
                ))),
            })),
        }),
    ]);

    let result = Signature {
        constness: None,
        asyncness: None,
        safety: Safety::Default,
        abi: None,
        fn_token: Token![fn](span),
        ident: Ident::new(fn_name, span),
        generics: Generics::default(),
        paren_token: token::Paren(span),
        inputs,
        variadic: None,
        output: ReturnType::Default,
    };
    return result;
}
