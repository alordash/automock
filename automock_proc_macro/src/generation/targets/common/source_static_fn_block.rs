use crate::common::*;
use not_enough_syntax::*;
use proc_macro2::Span;
use quote::format_ident;
use syn::*;

pub(crate) fn replace(
    span: Span,
    struct_path: &Path,
    item_impl: &mut ItemImpl,
    maybe_trait_path: Option<Path>,
) {
    let static_fns = item_impl
        .items
        .iter_mut()
        .filter_map(|x| match x {
            ImplItem::Fn(impl_item_fn) => Some(impl_item_fn),
            _ => None,
        })
        .filter(|x| !signature::is_associated(&x.sig));
    let qself = maybe_trait_path.as_ref().map(|trait_path| QSelf {
        lt_token: Token![<](span),
        ty: Box::new(Type::Path(TypePath {
            attrs: Vec::new(),
            qself: None,
            path: struct_path.clone(),
        })),
        position: trait_path.segments.len(),
        as_token: Some(Token![as](span)),
        gt_token: Token![>](span),
    });
    for static_fn in static_fns {
        normalization::normalize_struct_type_references_in_impl_item_fn(static_fn, struct_path);
        let args = static_fn
            .sig
            .inputs
            .iter_mut()
            .enumerate()
            .filter_map(|(i, fn_arg)| match fn_arg {
                FnArg::Typed(typed) => {
                    let arg_ident = format_ident!("__arg{i}");
                    let arg_path = ExprPath {
                        attrs: Vec::new(),
                        qself: None,
                        path: path::from_ident(arg_ident),
                    };
                    *typed.pat = Pat::Path(arg_path.clone());
                    Some(arg_path)
                }
                _ => None,
            })
            .map(Expr::Path)
            .collect();
        let func = ExprPath {
            attrs: Vec::new(),
            qself: qself.clone(),
            path: Path {
                leading_colon: None,
                segments: maybe_trait_path
                    .clone()
                    .map(|x| x.segments.into_iter())
                    .unwrap_or_else(|| struct_path.segments.clone().into_iter())
                    .chain(core::iter::once(PathSegment {
                        ident: static_fn.sig.ident.clone(),
                        arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                            colon2_token: Some(Token![::](span)),
                            lt_token: Token![<](span),
                            args: static_fn
                                .sig
                                .generics
                                .params
                                .iter()
                                .map(|x| generic_argument::from_param(x.clone()))
                                .collect(),
                            gt_token: Token![>](span),
                        }),
                    }))
                    .collect(),
            },
        };
        let call_stmt = ExprCall {
            attrs: Vec::new(),
            func: Box::new(Expr::Path(func)),
            paren_token: token::Paren(span),
            args,
        };
        static_fn.block = Block {
            brace_token: static_fn.block.brace_token,
            stmts: vec![Stmt::Expr(Expr::Call(call_stmt), None)],
        };
    }
}
