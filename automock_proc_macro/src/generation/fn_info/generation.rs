use crate::common::models::*;
use crate::generation::fn_info::models::*;
use crate::generation::fn_info::*;
use crate::preparation::r#fn::models::*;
use syn::*;

#[inline(always)]
pub(crate) fn generate(ctx: &Context, fn_syntax: FnSyntax) -> FnInfo {
    let generics = fn_syntax.merged_generics.clone();
    generate_core(ctx, fn_syntax, generics)
}

#[inline(always)]
pub(crate) fn generate_with_impl_generics(
    ctx: &Context,
    fn_syntax: FnSyntax,
    generics_for_impl: Generics,
) -> FnInfo {
    generate_core(ctx, fn_syntax, generics_for_impl)
}

fn generate_core(ctx: &Context, fn_syntax: FnSyntax, generics_for_impl: Generics) -> FnInfo {
    let call_struct = call_struct::generate(ctx, &fn_syntax, generics_for_impl.clone());
    let args_checker_struct = args_checker_struct::generate(
        &fn_syntax,
        Type::Path(TypePath {
            attrs: Vec::new(),
            qself: None,
            path: call_struct.path.clone(),
        }),
        generics_for_impl,
    );

    let result = FnInfo {
        spans: fn_syntax.spans,
        attributes: fn_syntax.attributes,
        source_signature: fn_syntax.source_signature,
        signature: fn_syntax.signature,
        visibility: fn_syntax.visibility,
        merged_generics: fn_syntax.merged_generics,
        maybe_owner_name: fn_syntax.maybe_owner_name,
        fn_ident: fn_syntax.fn_ident,
        fn_data_name: fn_syntax.fn_data_name,
        maybe_self_type: fn_syntax.maybe_self_type,
        arguments: fn_syntax.arguments,
        arg_refs_tuple: fn_syntax.arg_refs_tuple,
        maybe_base_impl: fn_syntax.maybe_base_impl,
        return_type: fn_syntax.return_type,
        call_struct,
        args_checker_struct,
    };

    return result;
}
