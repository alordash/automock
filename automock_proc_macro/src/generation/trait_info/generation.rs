use crate::common::models::*;
use crate::common::*;
use crate::generation::fn_info;
use crate::generation::fn_info::models::*;
use crate::generation::trait_info::models::*;
use crate::preparation::r#fn::models::*;
use crate::preparation::models::*;
use crate::preparation::r#trait::models::*;
use not_enough_syntax::*;
use syn::*;

pub(crate) fn generate(ctx: &Context, trait_syntax: TraitSyntax) -> TraitInfo {
    let span = trait_syntax.ident.span();
    let associated_items_info = AssociatedItemsInfo {
        trait_path_segment: PathSegment {
            ident: trait_syntax.ident.clone(),
            arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                colon2_token: Some(Token![::](span)),
                lt_token: Token![<](span),
                args: trait_syntax
                    .merged_generics
                    .params
                    .iter()
                    .map(|x| generic_argument::from_param(x.clone()))
                    .collect(),
                gt_token: Token![>](span),
            }),
        },
        associated_items_ident_strings: trait_syntax
            .constants
            .iter()
            .map(|x| x.item.ident.to_string())
            .chain(
                trait_syntax
                    .assoc_types
                    .iter()
                    .map(|x| x.item.ident.to_string()),
            )
            .collect(),
    };
    let result = TraitInfo {
        unsafety: trait_syntax.unsafety,
        merged_generics: trait_syntax.merged_generics,
        constants: trait_syntax.constants,
        assoc_types: trait_syntax.assoc_types,
        path: trait_syntax.path,
        static_fns: convert_fns(
            ctx,
            &trait_syntax.ident,
            &associated_items_info,
            trait_syntax.static_fns,
        ),
        associated_fns: convert_fns(
            ctx,
            &trait_syntax.ident,
            &associated_items_info,
            trait_syntax.associated_fns,
        ),
        ident: trait_syntax.ident,
        associated_items_info,
    };
    return result;
}

fn convert_fns(
    ctx: &Context,
    trait_ident: &Ident,
    associated_items_info: &AssociatedItemsInfo,
    fn_syntaxes: Vec<Ordered<FnSyntax>>,
) -> Vec<Ordered<FnInfo>> {
    fn_syntaxes
        .into_iter()
        .map(|ordered_fn_syntax| {
            ordered_fn_syntax.map(|fn_syntax| {
                let generics_for_impl =
                    generics::remove_defaults(fn_syntax.merged_generics.clone());
                // PERF - instead of normalizing results of `fn_syntax::generate` it could be faster
                // to normalize inputs to `fn_syntax::generate` first, i.e. build assoc items info
                // in `TraitSyntax` instead of `TraitInfo` and use it in `IFnOwner`
                let normalized_fn_syntax = normalization::normalize_associated_items_in_fn_syntax(
                    trait_ident,
                    associated_items_info,
                    fn_syntax,
                );
                return fn_info::generate_with_impl_generics(
                    ctx,
                    normalized_fn_syntax,
                    generics_for_impl.clone(),
                );
            })
        })
        .collect()
}
