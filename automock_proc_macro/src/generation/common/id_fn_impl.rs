use not_enough_syntax::*;
use proc_macro2::{Ident, Span};
use syn::*;

pub(crate) struct Params {
    pub public: bool,
}
pub(crate) fn new(span: Span, Params { public }: Params) -> ImplItemFn {
    let id_stmt = expr::method_call::new(
        span,
        Expr::Field(expr::field::new(
            Expr::Path(self_expr_path(span)),
            Ident::new("__mock_data", span),
        )),
        Ident::new("id", span),
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
            ident: Ident::new("id", span),
            generics: Generics::default(),
            paren_token: token::Paren(span),
            inputs: punctuated([ref_self_fn_arg(span)]),
            variadic: None,
            output: ReturnType::Type(
                Token!(->)(span),
                Box::new(Type::Path(r#type::path::new(span, ["usize"]))),
            ),
        },
        block: Block {
            brace_token: token::Brace(span),
            stmts: vec![Stmt::Expr(Expr::MethodCall(id_stmt), None)],
        },
    };
    return result;
}
