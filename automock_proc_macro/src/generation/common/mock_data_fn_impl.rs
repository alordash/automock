use crate::common::*;
use not_enough_syntax::*;
use proc_macro2::{Ident, Span};
use syn::*;

pub(crate) struct Params {
    pub public: bool,
}

pub(crate) fn new(span: Span, Params { public }: Params) -> ImplItemFn {
    let clone_expr = expr::method_call::new(
        span,
        Expr::Field(data_field::new_expr(span)),
        Ident::new("clone", span),
        [],
    );
    let result = ImplItemFn {
        attrs: Vec::new(),
        vis: if public {
            Visibility::Public(Token![pub](span))
        } else {
            Visibility::Inherited
        },
        modifiers: FnModifiers::default(),
        sig: Signature {
            constness: None,
            asyncness: None,
            safety: Safety::Default,
            abi: None,
            fn_token: Token![fn](span),
            ident: Ident::new("mock_data", span),
            generics: Generics::default(),
            paren_token: token::Paren(span),
            inputs: punctuated([ref_self_fn_arg(span)]),
            variadic: None,
            output: ReturnType::Type(
                Token![->](span),
                Box::new(Type::Path(TypePath {
                    attrs: Vec::new(),
                    qself: None,
                    path: path::new_global(span, for_generated::new("SharedMockData")),
                })),
            ),
        },
        block: Block {
            brace_token: token::Brace(span),
            stmts: vec![Stmt::Expr(Expr::MethodCall(clone_expr), None)],
        },
    };
    return result;
}
