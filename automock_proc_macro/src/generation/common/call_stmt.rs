use crate::common::*;
use crate::generation::fn_info::models::*;
use not_enough_syntax::*;
use proc_macro2::Span;
use syn::spanned::Spanned;
use syn::*;

pub(crate) struct Result {
    pub impl_trait_cast_stmts: Vec<Local>,
    pub call_var_path: ExprPath,
    pub call_stmt: Local,
}
pub(crate) fn new(span: Span, fn_info: &FnInfo, maybe_mod_ident: Option<Ident>) -> Result {
    let fn_data_var_path = expr::path::new(span, ["call"]);
    let mut call_struct_path = fn_info.call_struct.path.clone();
    if let Some(mod_ident) = maybe_mod_ident {
        call_struct_path.segments.insert(
            0,
            PathSegment {
                ident: mod_ident,
                arguments: PathArguments::None,
            },
        );
    }
    patch_lifetime::revert_in_first_generic_arg(&mut call_struct_path);
    let impl_trait_cast_stmts: Vec<_> = fn_info
        .source_signature
        .inputs
        .iter()
        .skip_while(|x| match x {
            FnArg::Receiver(_) => true,
            _ => false,
        })
        .map(|x| match x {
            FnArg::Typed(typed) => typed,
            _ => panic!(
                "All arguments after `FnArg::Receiver` should be of type `FnArg::Typed` only."
            ),
        })
        .zip(&fn_info.arguments)
        .filter_map(|(source_arg, arg)| {
            if arg.is_impl_trait {
                let span = source_arg.span();
                Some(Local {
                    attrs: Vec::new(),
                    let_token: Token![let](span),
                    modifiers: LocalModifiers::default(),
                    pat: Pat::Type(PatType {
                        attrs: Vec::new(),
                        pat: arg.ident_pat_type.pat.clone(),
                        colon_token: Token![:](span),
                        ty: Box::new(
                            normalization::replace_impl_trait_with_box_dyn_trait(
                                *source_arg.ty.clone(),
                            )
                            .ty,
                        ),
                    }),
                    init: Some(LocalInit {
                        eq_token: Token![=](span),
                        expr: Box::new(Expr::Call(expr::call::new(
                            span,
                            Expr::Path(expr::path::new(span, ["Box", "new"])),
                            [Expr::Path(ExprPath {
                                attrs: Vec::new(),
                                qself: None,
                                path: path::from_ident(arg.ident.clone()),
                            })],
                        ))),
                        diverge: None,
                    }),
                    semi_token: Token![;](span),
                })
            } else {
                None
            }
        })
        .collect();
    let fn_data_stmt = Local {
        attrs: Vec::new(),
        let_token: Token![let](span),
        modifiers: LocalModifiers::default(),
        pat: Pat::Path(fn_data_var_path.clone()),
        init: Some(LocalInit {
            eq_token: Token![=](span),
            expr: Box::new(Expr::Struct(ExprStruct {
                attrs: Vec::new(),
                qself: None,
                path: call_struct_path,
                brace_token: token::Brace(span),
                fields: [generics_field::new_value(span)]
                    .into_iter()
                    .chain(fn_info.arguments.iter().map(|x| FieldValue {
                        attrs: Vec::new(),
                        member: Member::Named(x.ident.clone()),
                        colon_token: Some(Token![:](span)),
                        expr: Expr::Macro(transmute_lifetime_expr::new(Expr::Path(ExprPath {
                            attrs: Vec::new(),
                            qself: None,
                            path: path::from_ident(x.ident.clone()),
                        }))),
                    }))
                    .collect(),
                dot2_token: None,
                rest: None,
            })),
            diverge: None,
        }),
        semi_token: Token![;](span),
    };
    let result = Result {
        impl_trait_cast_stmts,
        call_var_path: fn_data_var_path,
        call_stmt: fn_data_stmt,
    };
    return result;
}
