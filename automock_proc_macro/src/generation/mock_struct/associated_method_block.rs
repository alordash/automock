use crate::common::models::*;
use crate::generation::common::*;
use crate::generation::fn_info::models::*;
use crate::generation::mock_struct::common::*;
use crate::generation::mock_struct::models::*;
use proc_macro2::Span;
use syn::*;

pub(crate) struct Params<'a> {
    pub mock_struct_path: Path,
    pub fn_info: &'a FnInfo,
    pub maybe_base_fn_ident: Option<Ident>,
    pub mod_ident: Ident,
    pub maybe_base_trait_ident: Option<Ident>,
    pub qualify_call: bool,
}
pub(crate) fn generate(
    ctx: &Context,
    span: Span,
    Params {
        mock_struct_path,
        fn_info,
        maybe_base_fn_ident,
        mod_ident,
        maybe_base_trait_ident,
        qualify_call,
    }: Params,
) -> Block {
    let generic_arguments = generic_arguments::new(
        ctx,
        span,
        generic_arguments::Params {
            mock_struct_path: mock_struct_path.clone(),
            fn_info,
            remove_lifetime_generic_arguments: true,
        },
    );
    let maybe_use_base_trait = maybe_base_trait_ident.map(|x| {
        Item::Use(mod_usage::new(
            Visibility::Inherited,
            mod_ident.clone(),
            [x],
        ))
    });
    let call_stmt::Result {
        impl_trait_cast_stmts,
        call_var_path,
        call_stmt,
    } = call_stmt::new(span, fn_info, qualify_call.then_some(mod_ident));
    let (fn_data_var_path, fn_data_stmt) = fn_data_stmt::new_associated(
        span,
        fn_data_stmt::AssociatedParams {
            fn_info,
            generic_arguments,
            generics_info_provider_var_path: call_var_path.clone(),
        },
    );
    let fn_handle_stmt = fn_handle_stmt::generate(
        ctx,
        span,
        fn_handle_stmt::Params {
            fn_info,
            base_fn_kind: maybe_base_fn_ident
                .map(BaseFnKind::Associated)
                .unwrap_or(BaseFnKind::None),
            call_var_path,
            fn_data_var_path,
            is_static: false,
        },
    );

    let result = Block {
        brace_token: token::Brace(span),
        stmts: impl_trait_cast_stmts
            .into_iter()
            .map(Stmt::Local)
            .chain(maybe_use_base_trait.into_iter().map(Stmt::Item))
            .chain([
                Stmt::Local(call_stmt),
                Stmt::Local(fn_data_stmt),
                Stmt::Expr(fn_handle_stmt, None),
            ])
            .collect(),
    };
    return result;
}
