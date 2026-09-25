use crate::common::models::Context;
use crate::generation::fn_info;
use crate::generation::impl_struct_info::models::*;
use crate::preparation::r#struct::models::*;

pub(crate) fn generate(ctx: &Context, impl_struct_syntax: ImplStructSyntax) -> ImplStructInfo {
    let result = ImplStructInfo {
        attributes: impl_struct_syntax.attributes,
        target_ident: impl_struct_syntax.target_ident,
        target_path: impl_struct_syntax.target_path,
        generics: impl_struct_syntax.generics,
        static_fns: impl_struct_syntax
            .static_fns
            .into_iter()
            .map(|ordered| ordered.map(|x| fn_info::generate(ctx, x)))
            .collect(),
        associated_fns: impl_struct_syntax
            .associated_fns
            .into_iter()
            .map(|ordered| ordered.map(|x| fn_info::generate(ctx, x)))
            .collect(),
    };
    return result;
}
