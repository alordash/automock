use crate::common::models::Context;
use crate::generation::fn_info;
use crate::generation::impl_trait_for_struct_info::models::*;
use crate::preparation::r#struct::models::*;

pub(crate) fn generate(
    ctx: &Context,
    impl_trait_for_struct_syntax: ImplTraitForStructSyntax,
) -> ImplTraitForStructInfo {
    let result = ImplTraitForStructInfo {
        attributes: impl_trait_for_struct_syntax.attributes,
        target_path: impl_trait_for_struct_syntax.target_path,
        trait_ident: impl_trait_for_struct_syntax.trait_ident,
        trait_path: impl_trait_for_struct_syntax.trait_path,
        merged_generics: impl_trait_for_struct_syntax.merged_generics,
        target_simple_generics: impl_trait_for_struct_syntax.target_simple_generics,
        as_trait_where_predicates: impl_trait_for_struct_syntax.as_trait_where_predicates,
        trait_simple_generics: impl_trait_for_struct_syntax.trait_simple_generics,
        constants: impl_trait_for_struct_syntax.constants,
        types: impl_trait_for_struct_syntax.types,
        static_fns: impl_trait_for_struct_syntax
            .static_fns
            .into_iter()
            .map(|ordered| ordered.map(|x| fn_info::generate(ctx, x)))
            .collect(),
        associated_fns: impl_trait_for_struct_syntax
            .associated_fns
            .into_iter()
            .map(|ordered| ordered.map(|x| fn_info::generate(ctx, x)))
            .collect(),
    };
    return result;
}
