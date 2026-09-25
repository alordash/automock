use crate::common::*;
use crate::generation::mock_controls::models::*;
use not_enough_syntax::*;
use proc_macro2::Span;
use quote::format_ident;
use syn::*;

pub(crate) struct Params<'a> {
    pub struct_path: &'a Path,
    pub struct_generics: Generics,
    pub trait_control_struct_path: &'a Path,
    pub as_trait_where_predicates: &'a [WherePredicate],
    pub trait_ident: &'a Ident,
    pub trait_generics: Generics,
    pub maybe_common_where_clause: Option<WhereClause>,
    pub control_type: ControlType,
    pub is_static: bool,
}
pub(crate) fn generate(
    span: Span,
    Params {
        struct_path,
        struct_generics,
        trait_control_struct_path,
        as_trait_where_predicates,
        trait_ident,
        mut trait_generics,
        maybe_common_where_clause,
        control_type,
        is_static,
    }: Params,
) -> ItemImpl {
    let mut control_struct_ident_suffix = match control_type {
        ControlType::Setup => "Setup",
        ControlType::Received => "Received",
    }
    .to_string();
    if is_static {
        control_struct_ident_suffix = format!("Static{control_struct_ident_suffix}")
    };
    let self_ty_path = TypePath {
        attrs: Vec::new(),
        qself: None,
        path: path::from_base_path_with_ident(
            struct_path,
            format_ident!(
                "{}{}",
                path::last_ident(struct_path),
                control_struct_ident_suffix
            ),
        ),
    };
    trait_generics.where_clause = maybe_common_where_clause;
    let trait_generics_where_predicates = &mut trait_generics.make_where_clause().predicates;
    for extracted_where_predicate in as_trait_where_predicates {
        trait_generics_where_predicates.push(extracted_where_predicate.clone());
    }
    let fn_as_trait = generate_fn_as_trait(
        span,
        trait_control_struct_path,
        trait_ident,
        trait_generics,
        is_static,
    );
    let result = ItemImpl {
        attrs: Vec::new(),
        modifiers: ImplModifiers::default(),
        unsafety: None,
        impl_token: Token![impl](span),
        generics: struct_generics,
        trait_: None,
        self_ty: Box::new(Type::Path(self_ty_path)),
        brace_token: token::Brace(span),
        items: vec![ImplItem::Fn(fn_as_trait)],
    };
    return result;
}

fn generate_fn_as_trait(
    span: Span,
    trait_control_struct_path: &Path,
    trait_ident: &Ident,
    trait_generics: Generics,
    is_static: bool,
) -> ImplItemFn {
    let result = ImplItemFn {
        attrs: Vec::new(),
        vis: Visibility::Public(Token![pub](span)),
        modifiers: FnModifiers::default(),
        sig: Signature {
            constness: None,
            asyncness: None,
            safety: Safety::Default,
            abi: None,
            fn_token: Token![fn](span),
            ident: format_ident!("as_{trait_ident}"),
            generics: trait_generics,
            paren_token: token::Paren(span),
            inputs: punctuated([ref_self_fn_arg(span)]),
            variadic: None,
            output: ReturnType::Type(
                Token![->](span),
                Box::new(Type::Path(TypePath {
                    attrs: Vec::new(),
                    qself: None,
                    path: trait_control_struct_path.clone(),
                })),
            ),
        },
        block: Block {
            brace_token: token::Brace(span),
            stmts: vec![Stmt::Expr(
                Expr::Struct(ExprStruct {
                    attrs: Vec::new(),
                    qself: None,
                    path: trait_control_struct_path.clone(),
                    brace_token: token::Brace(span),
                    fields: if is_static {
                        punctuated([generics_field::new_value(span)])
                    } else {
                        punctuated([
                            data_field::new_clone_value(span),
                            generics_field::new_value(span),
                        ])
                    },
                    dot2_token: None,
                    rest: None,
                }),
                None,
            )],
        },
    };
    return result;
}
