use crate::common::*;
use crate::generation::common::reset_fn_data_stmt;
use crate::generation::mock_controls::models::*;
use not_enough_syntax::*;
use proc_macro2::Span;
use syn::punctuated::Punctuated;
use syn::*;

pub(crate) struct Params {
    pub struct_ident: Ident,
    pub generics: Generics,
    pub setup_struct_ident: Ident,
    pub received_struct_ident: Ident,
    pub static_setup_struct_ident: Ident,
    pub static_received_struct_ident: Ident,
}
pub(crate) fn generate(
    span: Span,
    Params {
        struct_ident,
        generics,
        setup_struct_ident,
        received_struct_ident,
        static_setup_struct_ident,
        static_received_struct_ident,
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
    let received_struct_type = Type::Path(TypePath {
        attrs: Vec::new(),
        qself: None,
        path: path::from_ident_with_generics(received_struct_ident, &generics),
    });
    let static_setup_struct_type = Type::Path(TypePath {
        attrs: Vec::new(),
        qself: None,
        path: path::from_ident_with_generics(static_setup_struct_ident, &generics),
    });
    let static_received_struct_type = Type::Path(TypePath {
        attrs: Vec::new(),
        qself: None,
        path: path::from_ident_with_generics(static_received_struct_ident, &generics),
    });
    let type_setup = associated_type_impl(span, "Setup", setup_struct_type);
    let fn_setup = fn_control(span, ControlType::Setup);
    let type_received = associated_type_impl(span, "Received", received_struct_type);
    let fn_received = fn_control(span, ControlType::Received);
    let type_static_setup = associated_type_impl(span, "StaticSetup", static_setup_struct_type);
    let fn_static_setup = fn_static_control(
        span,
        StaticControlType::Setup {
            mock_generic_argument: Box::new(GenericArgument::Type(struct_type.clone())),
        },
    );
    let type_static_received =
        associated_type_impl(span, "StaticReceived", static_received_struct_type);
    let fn_static_received = fn_static_control(span, StaticControlType::Received);
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
            ImplItem::Type(type_setup),
            ImplItem::Fn(fn_setup),
            ImplItem::Type(type_received),
            ImplItem::Fn(fn_received),
            ImplItem::Type(type_static_setup),
            ImplItem::Fn(fn_static_setup),
            ImplItem::Type(type_static_received),
            ImplItem::Fn(fn_static_received),
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

fn fn_control(span: Span, control_type: ControlType) -> ImplItemFn {
    let (fn_name, control_name) = match control_type {
        ControlType::Setup => ("setup", "Setup"),
        ControlType::Received => ("received", "Received"),
    };
    let control_path = path::new(span, ["Self", control_name]);
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
            ident: Ident::new(fn_name, span),
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

fn fn_static_control(span: Span, static_control_type: StaticControlType) -> ImplItemFn {
    let (fn_name, static_control_name) = match static_control_type {
        StaticControlType::Setup { .. } => ("static_setup", "StaticSetup"),
        StaticControlType::Received => ("static_received", "StaticReceived"),
    };
    let static_control_path = path::new(span, ["Self", static_control_name]);
    let constructor_stmt = Expr::Struct(ExprStruct {
        attrs: Vec::new(),
        qself: None,
        path: static_control_path.clone(),
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
            ident: Ident::new(fn_name, span),
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
