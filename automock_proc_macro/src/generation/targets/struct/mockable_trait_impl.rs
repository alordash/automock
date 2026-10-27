use crate::common::*;
use crate::generation::common::*;
use crate::generation::mock_struct::common::*;
use not_enough_syntax::*;
use proc_macro2::Span;
use syn::punctuated::Punctuated;
use syn::*;

pub(crate) struct Params {
    pub struct_ident: Ident,
    pub generics: Generics,
    pub setup_struct_ident: Ident,
    pub static_setup_struct_ident: Ident,
}
pub(crate) fn generate(
    span: Span,
    Params {
        struct_ident,
        generics,
        setup_struct_ident,
        static_setup_struct_ident,
    }: Params,
) -> ItemImpl {
    let mut struct_generics = generics.clone();
    struct_generics.params = struct_generics.params.into_iter().skip(1).collect(); // skipping '__ama
    let struct_path = path::from_ident_with_generics(struct_ident, &struct_generics);
    let struct_type = Type::Path(TypePath {
        attrs: Vec::new(),
        qself: None,
        path: struct_path.clone(),
    });
    let setup_struct_type = Type::Path(TypePath {
        attrs: Vec::new(),
        qself: None,
        path: path::from_ident_with_generics(setup_struct_ident, &generics),
    });
    let static_setup_struct_type = Type::Path(TypePath {
        attrs: Vec::new(),
        qself: None,
        path: path::from_ident_with_generics(static_setup_struct_ident, &generics),
    });
    let id_impl = id_fn_impl::new(span, id_fn_impl::Params { public: false });
    let mock_data_impl = mock_data_fn_impl::new(span, mock_data_fn_impl::Params { public: false });
    let type_setup = associated_type_impl(span, "Setup", setup_struct_type);
    let fn_setup = fn_setup(span);
    let type_static_setup = associated_type_impl(span, "StaticSetup", static_setup_struct_type);
    let fn_static_setup = fn_static_setup(span, GenericArgument::Type(struct_type.clone()));
    let fn_verify_calls = verify_calls_fn::generate(
        span,
        verify_calls_fn::Params {
            for_mockable_trait: true,
        },
    );
    let fn_verify_static_calls = verify_calls_fn::generate_static(
        span,
        verify_calls_fn::StaticParams {
            mock_struct_path: struct_path,
            for_mockable_trait: true,
        },
    );
    let result = ItemImpl {
        attrs: Vec::new(),
        modifiers: ImplModifiers::default(),
        unsafety: None,
        impl_token: Token![impl](span),
        generics,
        trait_: Some((
            path::new_generics_global(
                span,
                ["automock", "Mockable"],
                [GenericArgument::Lifetime(patch_lifetime::new(span))],
            ),
            Token![for](span),
        )),
        self_ty: Box::new(struct_type),
        brace_token: token::Brace(span),
        items: vec![
            ImplItem::Fn(id_impl),
            ImplItem::Fn(mock_data_impl),
            ImplItem::Type(type_setup),
            ImplItem::Fn(fn_setup),
            ImplItem::Type(type_static_setup),
            ImplItem::Fn(fn_static_setup),
            ImplItem::Fn(fn_verify_calls),
            ImplItem::Fn(fn_verify_static_calls),
        ],
    };
    return result;
}

fn associated_type_impl(span: Span, name: &'static str, ty: Type) -> ImplItemType {
    let result = ImplItemType {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        modifiers: TypeModifiers::default(),
        type_token: Token![type](span),
        ident: Ident::new(name, span),
        generics: Generics::default(),
        eq_token: Token![=](span),
        ty,
        semi_token: Token![;](span),
    };
    return result;
}

fn fn_setup(span: Span) -> ImplItemFn {
    let control_path = path::new(span, ["Self", "Setup"]);
    let result = ImplItemFn {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        modifiers: FnModifiers::default(),
        sig: Signature {
            constness: None,
            asyncness: None,
            safety: Safety::Default,
            abi: None,
            fn_token: Token![fn](span),
            ident: Ident::new("setup", span),
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
        },
        block: Block {
            brace_token: token::Brace(span),
            stmts: vec![Stmt::Expr(
                Expr::Struct(ExprStruct {
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
                }),
                None,
            )],
        },
    };
    return result;
}

fn fn_static_setup(span: Span, mock_generic_argument: GenericArgument) -> ImplItemFn {
    let static_control_path = path::new(span, ["Self", "StaticSetup"]);
    let constructor_stmt = Expr::Struct(ExprStruct {
        attrs: Vec::new(),
        qself: None,
        path: static_control_path.clone(),
        brace_token: token::Brace(span),
        fields: punctuated([generics_field::new_value(span)]),
        dot2_token: None,
        rest: None,
    });
    let reset_fn_data_stmt = reset_fn_data_stmt::new(span, mock_generic_argument);
    let stmts = vec![
        Stmt::Expr(Expr::Call(reset_fn_data_stmt), Some(Token![;](span))),
        Stmt::Expr(constructor_stmt, None),
    ];
    let result = ImplItemFn {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        modifiers: FnModifiers::default(),
        sig: Signature {
            constness: None,
            asyncness: None,
            safety: Safety::Default,
            abi: None,
            fn_token: Token![fn](span),
            ident: Ident::new("static_setup", span),
            generics: Generics::default(),
            paren_token: token::Paren(span),
            inputs: Punctuated::new(),
            variadic: None,
            output: ReturnType::Type(
                Token![->](span),
                Box::new(Type::Path(TypePath {
                    attrs: Vec::new(),
                    qself: None,
                    path: static_control_path,
                })),
            ),
        },
        block: Block {
            brace_token: token::Brace(span),
            stmts,
        },
    };
    return result;
}
