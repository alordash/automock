use crate::common::*;
use crate::generation::fn_info::*;
use crate::preparation::r#fn::models::*;
use not_enough_syntax::*;
use proc_macro2::Span;
use quote::ToTokens;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::*;

pub(crate) fn generate(
    span: Span,
    generics: Generics,
    arguments: &[Argument],
    target_type: Type,
    call_struct_type: Type,
) -> ItemImpl {
    let items = if arguments.is_empty() {
        Vec::new()
    } else {
        let fn_check = generate_fn_check(span, arguments, call_struct_type);
        let fn_fmt_args = generate_fn_fmt_args(span, arguments);
        vec![ImplItem::Fn(fn_check), ImplItem::Fn(fn_fmt_args)]
    };

    let result = ItemImpl {
        attrs: Vec::new(),
        modifiers: ImplModifiers::default(),
        unsafety: None,
        impl_token: Token![impl](span),
        generics,
        trait_: Some((
            path::new_global(span, rsubstitute_for_generated::new("IArgsChecker")),
            Token![for](span),
        )),
        self_ty: Box::new(target_type),
        brace_token: token::Brace(span),
        items,
    };

    return result;
}

fn generate_fn_check(span: Span, arguments: &[Argument], call_struct_type: Type) -> ImplItemFn {
    let dyn_call_arg_path = path::new(span, ["dyn_call"]);
    let sig = Signature {
        constness: None,
        asyncness: None,
        safety: Safety::Default,
        abi: None,
        fn_token: Token![fn](span),
        ident: Ident::new("check", span),
        generics: Generics::default(),
        paren_token: token::Paren(span),
        inputs: punctuated([
            ref_self_fn_arg(span),
            FnArg::Typed(PatType {
                attrs: Vec::new(),
                pat: Box::new(Pat::Path(PatPath {
                    attrs: Vec::new(),
                    qself: None,
                    path: dyn_call_arg_path.clone(),
                })),
                colon_token: Token![:](span),
                ty: Box::new(Type::Reference(TypeReference {
                    attrs: Vec::new(),
                    and_token: Token![&](span),
                    lifetime: None,
                    mutability: None,
                    elem: Box::new(Type::Path(TypePath {
                        attrs: Vec::new(),
                        qself: None,
                        path: path::new_generics_global(
                            span,
                            rsubstitute_for_generated::new("DynCall"),
                            [anonymous_lifetime_generic_argument(span)],
                        ),
                    })),
                })),
            }),
        ]),
        variadic: None,
        output: ReturnType::Type(
            Token![->](span),
            Box::new(Type::Path(r#type::vec_of(
                span,
                Type::Path(r#type::path::new_global(
                    span,
                    rsubstitute_for_generated::new("ArgCheckResult"),
                )),
            ))),
        ),
    };

    let use_i_debug_printer_stmt = rsubstitute_for_generated::glob_usage(span, "arg_printing");
    let call_path = path::new(span, ["call"]);
    let call_stmt = Stmt::Local(Local {
        attrs: Vec::new(),
        let_token: Token![let](span),
        modifiers: LocalModifiers::default(),
        pat: Pat::Type(PatType {
            attrs: Vec::new(),
            pat: Box::new(Pat::Path(PatPath {
                attrs: Vec::new(),
                qself: None,
                path: call_path.clone(),
            })),
            colon_token: Token![:](span),
            ty: Box::new(Type::Reference(TypeReference {
                attrs: Vec::new(),
                and_token: Token![&](span),
                lifetime: None,
                mutability: None,
                elem: Box::new(call_struct_type),
            })),
        }),
        init: Some(LocalInit {
            eq_token: Token![=](span),
            expr: Box::new(Expr::MethodCall(expr::method_call::new(
                span,
                Expr::Path(ExprPath {
                    attrs: Vec::new(),
                    qself: None,
                    path: dyn_call_arg_path,
                }),
                Ident::new("downcast_ref", span),
                [],
            ))),
            diverge: None,
        }),
        semi_token: Token![;](span),
    });
    let vec_stmt_exprs: Punctuated<Expr, Token![,]> = arguments
        .iter()
        .map(|argument| {
            let span = argument.ident_pat_type.span();
            let call_field = Expr::Field(expr::field::new(
                Expr::Path(ExprPath {
                    attrs: Vec::new(),
                    qself: None,
                    path: call_path.clone(),
                }),
                argument.ident.clone(),
            ));
            let result = expr::method_call::new(
                span,
                Expr::Macro(transmute_lifetime_expr::new_with_target(
                    Expr::Reference(ExprReference {
                        attrs: Vec::new(),
                        and_token: Token![&](span),
                        mutability: None,
                        expr: Box::new(Expr::Field(expr::field::new_self(argument.ident.clone()))),
                    }),
                    Type::Reference(TypeReference {
                        attrs: Vec::new(),
                        and_token: Token![&](span),
                        lifetime: None,
                        mutability: None,
                        elem: Box::new(Type::Path(arg_type::of(
                            span,
                            *argument.ident_pat_type.ty.clone(),
                        ))),
                    }),
                )),
                Ident::new("check", span),
                [
                    Expr::Lit(ExprLit {
                        attrs: Vec::new(),
                        lit: Lit::Str(LitStr::new(&argument.ident.to_string(), span)),
                    }),
                    Expr::Macro(transmute_lifetime_expr::new(Expr::Reference(
                        ExprReference {
                            attrs: Vec::new(),
                            and_token: Token![&](span),
                            mutability: None,
                            expr: Box::new(call_field.clone()),
                        },
                    ))),
                    arg_printer_expr::new(span, call_field, *argument.ident_pat_type.ty.clone()),
                ],
            );
            return Expr::MethodCall(result);
        })
        .collect();
    let vec_stmt = Stmt::Expr(
        Expr::Macro(ExprMacro {
            attrs: Vec::new(),
            mac: r#macro::vec(span, vec_stmt_exprs.to_token_stream()),
        }),
        None,
    );
    let block = Block {
        brace_token: token::Brace(span),
        stmts: vec![
            Stmt::Item(Item::Use(use_i_debug_printer_stmt)),
            call_stmt,
            vec_stmt,
        ],
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

fn generate_fn_fmt_args(span: Span, arguments: &[Argument]) -> ImplItemFn {
    let sig = Signature {
        constness: None,
        asyncness: None,
        safety: Safety::Default,
        abi: None,
        fn_token: Token![fn](span),
        ident: Ident::new("fmt_args", span),
        generics: Generics::default(),
        paren_token: token::Paren(span),
        inputs: punctuated([ref_self_fn_arg(span)]),
        variadic: None,
        output: ReturnType::Type(
            Token![->](span),
            Box::new(Type::Path(r#type::path::new(span, ["String"]))),
        ),
    };

    let use_i_debug_printer_stmt = rsubstitute_for_generated::glob_usage(span, "arg_printing");
    let format_template_lit = Expr::Lit(ExprLit {
        attrs: Vec::new(),
        lit: Lit::Str(LitStr::new(&vec!["{}"; arguments.len()].join(", "), span)),
    });
    let arg_print_exprs = arguments.iter().map(|argument| {
        arg_printer_expr::new(
            span,
            Expr::Reference(ExprReference {
                attrs: Vec::new(),
                and_token: Token![&](span),
                mutability: None,
                expr: Box::new(Expr::Field(expr::field::new_self(argument.ident.clone()))),
            }),
            Type::Reference(TypeReference {
                attrs: Vec::new(),
                and_token: Token![&](span),
                lifetime: None,
                mutability: None,
                elem: Box::new(Type::Path(arg_type::of(
                    span,
                    *argument.ident_pat_type.ty.clone(),
                ))),
            }),
        )
    });

    let format_expr_args: Punctuated<Expr, Token![,]> = core::iter::once(format_template_lit)
        .chain(arg_print_exprs)
        .collect();
    let format_expr = ExprMacro {
        attrs: Vec::new(),
        mac: Macro {
            path: path::new(span, ["format"]),
            bang_token: Token![!](span),
            delimiter: MacroDelimiter::Paren(token::Paren(span)),
            tokens: format_expr_args.to_token_stream(),
        },
    };
    let block = Block {
        brace_token: token::Brace(span),
        stmts: vec![
            Stmt::Item(Item::Use(use_i_debug_printer_stmt)),
            Stmt::Expr(Expr::Macro(format_expr), None),
        ],
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
