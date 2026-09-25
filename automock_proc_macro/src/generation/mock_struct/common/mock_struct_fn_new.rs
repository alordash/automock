use crate::common::*;
use not_enough_syntax::*;
use proc_macro2::Span;
use syn::punctuated::Punctuated;
use syn::*;

pub(crate) fn new(span: Span) -> ImplItemFn {
    let sig = Signature {
        constness: None,
        asyncness: None,
        safety: Safety::Default,
        abi: None,
        fn_token: Token![fn](span),
        ident: Ident::new("new", span),
        generics: Generics::default(),
        paren_token: token::Paren(span),
        inputs: Punctuated::new(),
        variadic: None,
        output: ReturnType::Type(Token![->](span), Box::new(Type::Path(self_type(span)))),
    };
    let constructor_stmt = Expr::Struct(ExprStruct {
        attrs: Vec::new(),
        qself: None,
        path: self_type_path(span),
        brace_token: token::Brace(span),
        fields: punctuated([
            generics_field::new_value(span),
            data_field::new_default_value(span),
        ]),
        dot2_token: None,
        rest: None,
    });
    let block = Block {
        brace_token: token::Brace(span),
        stmts: vec![Stmt::Expr(constructor_stmt, None)],
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
