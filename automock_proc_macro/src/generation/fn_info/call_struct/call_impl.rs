use crate::common::*;
use crate::generation::fn_info::*;
use crate::preparation::r#fn::models::*;
use not_enough_syntax::r#type::vec_of;
use not_enough_syntax::*;
use proc_macro2::Span;
use quote::ToTokens;
use syn::punctuated::Punctuated;
use syn::*;

pub(crate) fn generate(
    span: Span,
    generics: Generics,
    arguments: &[Argument],
    target_type: Type,
) -> ItemImpl {
    let items = if arguments.is_empty() {
        Vec::new()
    } else {
        let fn_get_arg_infos = generate_fn_get_args_infos(span, arguments);
        let fn_get_ptr_to_boxed_tuple_of_refs =
            generate_fn_get_ptr_to_boxed_tuple_of_refs(span, arguments);
        vec![
            ImplItem::Fn(fn_get_arg_infos),
            ImplItem::Fn(fn_get_ptr_to_boxed_tuple_of_refs),
        ]
    };

    let result = ItemImpl {
        attrs: Vec::new(),
        modifiers: ImplModifiers::default(),
        unsafety: None,
        impl_token: Token![impl](span),
        generics,
        trait_: Some((
            path::new_global(span, for_generated::new("ICall")),
            Token![for](span),
        )),
        self_ty: Box::new(target_type),
        brace_token: token::Brace(span),
        items,
    };

    return result;
}

fn generate_fn_get_args_infos(span: Span, arguments: &[Argument]) -> ImplItemFn {
    let sig = Signature {
        constness: None,
        asyncness: None,
        safety: Safety::Default,
        abi: None,
        fn_token: Token![fn](span),
        ident: Ident::new("get_arg_infos", span),
        generics: Generics::default(),
        paren_token: token::Paren(span),
        inputs: punctuated([ref_self_fn_arg(span)]),
        variadic: None,
        output: ReturnType::Type(
            Token![->](span),
            Box::new(Type::Path(vec_of(
                span,
                Type::Path(r#type::path::new_global(
                    span,
                    for_generated::new("ArgInfo"),
                )),
            ))),
        ),
    };

    let use_i_debug_printer_stmt = for_generated::glob_usage(span, "arg_printing");
    let arg_info_new_exprs: Punctuated<Expr, Token![,]> =
        arguments.iter().map(generate_arg_info_new_expr).collect();
    let vec_stmt = Stmt::Expr(
        Expr::Macro(ExprMacro {
            attrs: Vec::new(),
            mac: r#macro::vec(span, arg_info_new_exprs.to_token_stream()),
        }),
        None,
    );
    let block = Block {
        brace_token: token::Brace(span),
        stmts: vec![Stmt::Item(Item::Use(use_i_debug_printer_stmt)), vec_stmt],
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

fn generate_arg_info_new_expr(argument: &Argument) -> Expr {
    let span = argument.ident.span();

    let arg_name_argument = Expr::Lit(ExprLit {
        attrs: Vec::new(),
        lit: Lit::Str(LitStr::new(&argument.ident.to_string(), span)),
    });

    let arg_field_expr = Expr::Field(expr::field::new_self(argument.ident.clone()));
    let arg_value_argument = Expr::Reference(ExprReference {
        attrs: Vec::new(),
        and_token: Token![&](span),
        mutability: None,
        expr: Box::new(arg_field_expr.clone()),
    });

    let arg_debug_string_argument =
        arg_printer_expr::new(span, arg_field_expr, *argument.ident_pat_type.ty.clone());

    let result = Expr::Call(expr::call::new(
        span,
        Expr::Path(expr::path::new_global(
            span,
            for_generated::new2("ArgInfo", "new"),
        )),
        [
            arg_name_argument,
            arg_value_argument,
            arg_debug_string_argument,
        ],
    ));

    return result;
}

fn generate_fn_get_ptr_to_boxed_tuple_of_refs(span: Span, arguments: &[Argument]) -> ImplItemFn {
    let sig = Signature {
        constness: None,
        asyncness: None,
        safety: Safety::Default,
        abi: None,
        fn_token: Token![fn](span),
        ident: Ident::new("get_ptr_to_boxed_tuple_of_refs", span),
        generics: Generics::default(),
        paren_token: token::Paren(span),
        inputs: punctuated([ref_self_fn_arg(span)]),
        variadic: None,
        output: ReturnType::Type(Token![->](span), Box::new(mut_ptr_void_type(span))),
    };

    let fields: Punctuated<Expr, Token![,]> = arguments
        .iter()
        .map(|argument| {
            Expr::Reference(ExprReference {
                attrs: Vec::new(),
                and_token: Token![&](span),
                mutability: None,
                expr: Box::new(Expr::Field(expr::field::new_self(argument.ident.clone()))),
            })
        })
        .collect();
    let tuple = Expr::Tuple(ExprTuple {
        attrs: Vec::new(),
        paren_token: token::Paren(span),
        elems: fields,
    });
    let box_new = Expr::Call(expr::call::new(
        span,
        Expr::Path(expr::path::new(span, ["Box", "new"])),
        [tuple],
    ));
    let box_leak = Expr::Call(expr::call::new(
        span,
        Expr::Path(expr::path::new(span, ["Box", "leak"])),
        [box_new],
    ));
    let as_mut_infer = Expr::Cast(ExprCast {
        attrs: Vec::new(),
        expr: Box::new(box_leak),
        as_token: Token![as](span),
        ty: Box::new(mut_ptr_infer_type(span)),
    });
    let as_mut_void = Expr::Cast(ExprCast {
        attrs: Vec::new(),
        expr: Box::new(as_mut_infer),
        as_token: Token![as](span),
        ty: Box::new(mut_ptr_void_type(span)),
    });

    let block = Block {
        brace_token: token::Brace(span),
        stmts: vec![Stmt::Expr(as_mut_void, None)],
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
