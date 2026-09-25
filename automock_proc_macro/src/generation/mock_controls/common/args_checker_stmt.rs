use crate::common::*;
use crate::generation::fn_info::models::*;
use not_enough_syntax::*;
use proc_macro2::Span;
use syn::*;

const ARGUMENT_ARG_NAME: &str = "v";

pub(crate) fn new(span: Span, fn_info: &FnInfo) -> (ExprPath, Local) {
    let args_checker_var_path = expr::path::new(span, ["args_checker"]);
    let args_checker_stmt = Local {
        attrs: Vec::new(),
        let_token: Token![let](span),
        modifiers: LocalModifiers::default(),
        pat: Pat::Path(args_checker_var_path.clone()),
        init: Some(LocalInit {
            eq_token: Token![=](span),
            expr: Box::new(Expr::Struct(ExprStruct {
                attrs: Vec::new(),
                qself: None,
                path: fn_info.args_checker_struct.path.clone(),
                brace_token: token::Brace(span),
                fields: [generics_field::new_value(span)]
                    .into_iter()
                    .chain(fn_info.arguments.iter().map(|x| {
                        let arg_printer = Expr::Call(expr::call::new(
                            span,
                            Expr::Path(expr::path::new_global(
                                span,
                                for_generated::new("ArgPrinter"),
                            )),
                            [Expr::Path(expr::path::new(span, [ARGUMENT_ARG_NAME]))],
                        ));
                        let arg_printer_ref = Expr::Reference(ExprReference {
                            attrs: Vec::new(),
                            and_token: Token![&](span),
                            mutability: None,
                            expr: Box::new(arg_printer),
                        });
                        let arg_printer_ref_paren = Expr::Paren(ExprParen {
                            attrs: Vec::new(),
                            paren_token: token::Paren(span),
                            expr: Box::new(arg_printer_ref),
                        });
                        let arg_printer_expr = expr::method_call::new(
                            span,
                            arg_printer_ref_paren,
                            Ident::new("debug_string", span),
                            [],
                        );
                        let format_debug_string_closure = ExprClosure {
                            attrs: Vec::new(),
                            lifetimes: None,
                            modifiers: ClosureModifiers::default(),
                            constness: None,
                            asyncness: None,
                            capture: None,
                            inputs_begin: Token![|](span),
                            inputs: punctuated([Pat::Ident(PatIdent {
                                attrs: Vec::new(),
                                by_ref: None,
                                mutability: None,
                                ident: Ident::new(ARGUMENT_ARG_NAME, span),
                                subpat: None,
                            })]),
                            inputs_end: Token![|](span),
                            output: ReturnType::Default,
                            body: Box::new(Expr::Block(ExprBlock {
                                attrs: Vec::new(),
                                label: None,
                                block: Block {
                                    brace_token: token::Brace(span),
                                    stmts: vec![
                                        Stmt::Item(Item::Use(
                                            for_generated::glob_usage(
                                                span,
                                                "arg_printing",
                                            ),
                                        )),
                                        Stmt::Expr(Expr::MethodCall(arg_printer_expr), None),
                                    ],
                                },
                            })),
                        };
                        FieldValue {
                            attrs: Vec::new(),
                            member: Member::Named(x.ident.clone()),
                            colon_token: Some(Token![:](span)),
                            expr: Expr::Macro(transmute_lifetime_expr::new(Expr::MethodCall(
                                expr::method_call::new(
                                    span,
                                    Expr::Path(ExprPath {
                                        attrs: Vec::new(),
                                        qself: None,
                                        path: path::from_ident(x.ident.clone()),
                                    }),
                                    Ident::new("into_arg", span),
                                    [Expr::Closure(format_debug_string_closure)],
                                ),
                            ))),
                        }
                    }))
                    .collect(),
                dot2_token: None,
                rest: None,
            })),
            diverge: None,
        }),
        semi_token: Token![;](span),
    };
    return (args_checker_var_path, args_checker_stmt);
}
