use not_enough_syntax::*;
use proc_macro2::Span;
use syn::*;

pub(crate) fn generate(
    span: Span,
    generics: Generics,
    struct_path: Path,
    fields: &Fields,
) -> ItemImpl {
    let Fields::Named(fields_named) = fields else {
        panic!("`fields` for `Clone` implementation generation must be named.")
    };
    let target_type = TypePath {
        attrs: Vec::new(),
        qself: None,
        path: struct_path.clone(),
    };
    let sig = Signature {
        constness: None,
        asyncness: None,
        safety: Safety::Default,
        abi: None,
        fn_token: Token![fn](span),
        ident: Ident::new("clone", span),
        generics: Generics::default(),
        paren_token: token::Paren(span),
        inputs: punctuated([ref_self_fn_arg(span)]),
        variadic: None,
        output: ReturnType::Type(Token![->](span), Box::new(Type::Path(target_type.clone()))),
    };
    let target_stmt = ExprStruct {
        attrs: Vec::new(),
        qself: None,
        path: struct_path,
        brace_token: token::Brace(span),
        fields: fields_named
            .named
            .iter()
            .map(|x| {
                let field_ident = x
                    .ident
                    .clone()
                    .expect("Call struct fields should have identifiers");
                FieldValue {
                    attrs: Vec::new(),
                    member: Member::Named(field_ident.clone()),
                    colon_token: Some(Token![:](span)),
                    expr: Expr::Call(expr::call::new(
                        span,
                        Expr::Path(ExprPath {
                            attrs: Vec::new(),
                            qself: None,
                            path: path::new_global(span, ["core", "clone", "Clone", "clone"]),
                        }),
                        [Expr::Reference(ExprReference {
                            attrs: Vec::new(),
                            and_token: Token![&](span),
                            mutability: None,
                            expr: Box::new(Expr::Field(expr::field::new_self(field_ident))),
                        })],
                    )),
                }
            })
            .collect(),
        dot2_token: None,
        rest: None,
    };
    let block = Block {
        brace_token: token::Brace(span),
        stmts: vec![Stmt::Expr(Expr::Struct(target_stmt), None)],
    };

    let fn_clone = ImplItemFn {
        attrs: vec![attributes::inline(span)],
        vis: Visibility::Inherited,
        modifiers: FnModifiers::default(),
        sig,
        block,
    };

    let result = ItemImpl {
        attrs: Vec::new(),
        modifiers: ImplModifiers::default(),
        unsafety: None,
        impl_token: Token![impl](span),
        generics,
        trait_: Some((
            path::new_global(span, ["core", "clone", "Clone"]),
            Token![for](span),
        )),
        self_ty: Box::new(Type::Path(target_type)),
        brace_token: token::Brace(span),
        items: vec![ImplItem::Fn(fn_clone)],
    };
    return result;
}
