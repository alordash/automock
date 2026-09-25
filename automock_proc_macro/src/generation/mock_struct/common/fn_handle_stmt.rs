use crate::common::models::*;
use crate::generation::fn_info::models::*;
use crate::generation::mock_struct::models::*;
use not_enough_syntax::*;
use proc_macro2::Span;
use syn::*;

pub(crate) struct Params<'a> {
    pub fn_info: &'a FnInfo,
    pub base_fn_kind: BaseFnKind,
    pub call_var_path: ExprPath,
    pub fn_data_var_path: ExprPath,
    pub is_static: bool,
}
pub(crate) fn generate(
    ctx: &Context,
    span: Span,
    Params {
        fn_info,
        base_fn_kind,
        call_var_path,
        fn_data_var_path,
        is_static,
    }: Params,
) -> Expr {
    let mock_arg = if is_static {
        void_tuple(span)
    } else {
        Expr::Path(self_expr_path(span))
    };
    let maybe_base_fn_path = match (ctx.support_base_calling, base_fn_kind) {
        (true, BaseFnKind::StaticFn(base_fn_ident)) => {
            Some(generate_base_fn_path(span, fn_info, base_fn_ident))
        }
        (true, BaseFnKind::Associated(base_fn_ident)) => {
            let mut base_fn_path = generate_base_fn_path(span, fn_info, base_fn_ident);
            base_fn_path.segments.insert(
                0,
                PathSegment {
                    ident: self_type_ident(span),
                    arguments: PathArguments::None,
                },
            );
            Some(base_fn_path)
        }
        (_, BaseFnKind::None) | (false, _) => None,
    };
    let maybe_base_call = maybe_base_fn_path.map(|path| {
        Expr::Path(ExprPath {
            attrs: Vec::new(),
            qself: None,
            path,
        })
    });

    let args = if let Some(base_call) = maybe_base_call {
        [mock_arg, Expr::Path(call_var_path), base_call]
            .into_iter()
            .collect()
    } else {
        [mock_arg, Expr::Path(call_var_path)].into_iter().collect()
    };
    let method_ident = if fn_info.source_signature.asyncness.is_some() {
        "handle_async"
    } else {
        "handle"
    };
    let handle_expr = ExprMethodCall {
        attrs: Vec::new(),
        receiver: Box::new(Expr::Path(fn_data_var_path)),
        dot_token: Token![.](span),
        method: Ident::new(method_ident, span),
        turbofish: None,
        paren_token: token::Paren(span),
        args,
    };
    let result = if fn_info.source_signature.asyncness.is_some() {
        Expr::Await(ExprAwait {
            attrs: Vec::new(),
            base: Box::new(Expr::MethodCall(handle_expr)),
            dot_token: Token![.](span),
            await_token: Token![await](span),
        })
    } else {
        Expr::MethodCall(handle_expr)
    };
    return result;
}

fn generate_base_fn_path(span: Span, fn_info: &FnInfo, base_fn_ident: Ident) -> Path {
    let result = Path {
        leading_colon: None,
        segments: [PathSegment {
            ident: base_fn_ident,
            arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                colon2_token: Some(Token![::](span)),
                lt_token: Token![<](span),
                args: fn_info
                    .signature
                    .generics
                    .params
                    .iter()
                    .filter(|x| match x {
                        GenericParam::Lifetime(_) => false,
                        _ => true,
                    })
                    .cloned()
                    .map(generic_argument::from_param)
                    .collect(),
                gt_token: Token![>](span),
            }),
        }]
        .into_iter()
        .collect(),
    };
    return result;
}
