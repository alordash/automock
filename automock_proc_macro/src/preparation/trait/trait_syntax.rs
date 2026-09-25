use super::models::*;
use crate::common::{normalization, patch_lifetime};
use crate::preparation::r#fn::fn_syntax;
use crate::preparation::r#fn::models::*;
use crate::preparation::models::*;
use not_enough_syntax::*;
use quote::{ToTokens, format_ident};
use syn::*;

pub(crate) struct Params {
    pub unsafety: Option<Token![unsafe]>,
    pub ident: Ident,
    pub generics: Generics,
    pub items: Vec<TraitItem>,
}

pub(crate) fn prepare(
    Params {
        unsafety,
        ident,
        generics,
        items,
    }: Params,
) -> TraitSyntax {
    let mut split_items = split_items(items, &ident);
    let path = path::from_ident_with_generics(ident.clone(), &generics);
    let mut merged_generics = merge_generics_with_assoc_generics(
        &ident,
        generics,
        &split_items.assoc_types,
        &split_items.assoc_constants,
    );
    let trait_mock_path = path::from_ident_with_generics(
        format_ident!("{}Mock", ident),
        &patch_lifetime::prepend_to_generics(merged_generics.clone()),
    );
    split_items.associated_fns = split_items
        .associated_fns
        .into_iter()
        .map(|ordered| {
            ordered.map(|x| {
                normalization::normalize_struct_type_references_in_trait_item_fn(
                    x,
                    &trait_mock_path,
                )
            })
        })
        .collect();
    split_items.static_fns = split_items
        .static_fns
        .into_iter()
        .map(|ordered| {
            ordered.map(|x| {
                normalization::normalize_struct_type_references_in_trait_item_fn(
                    x,
                    &trait_mock_path,
                )
            })
        })
        .collect();
    let trait_syntax_as_fn_owner = TraitSyntaxAsFnOwner {
        trait_ident: &ident,
        generics: &merged_generics,
    };
    let static_fns = split_items
        .static_fns
        .into_iter()
        .map(|ordered| {
            ordered.map(|x| map_trait_item_fn_to_fn_syntax(x, &trait_syntax_as_fn_owner))
        })
        .collect();
    let associated_fns = split_items
        .associated_fns
        .into_iter()
        .map(|ordered| {
            ordered.map(|x| map_trait_item_fn_to_fn_syntax(x, &trait_syntax_as_fn_owner))
        })
        .collect();
    merged_generics = patch_lifetime::prepend_to_generics(merged_generics);

    let result = TraitSyntax {
        unsafety,
        ident,
        merged_generics,
        constants: split_items.assoc_constants,
        assoc_types: split_items.assoc_types,
        path,
        static_fns,
        associated_fns,
    };
    return result;
}

#[derive(Default)]
struct SplitItems {
    pub assoc_constants: Vec<Ordered<TraitItemConstSyntax>>,
    pub assoc_types: Vec<Ordered<TraitItemTypeSyntax>>,
    pub static_fns: Vec<Ordered<TraitItemFn>>,
    pub associated_fns: Vec<Ordered<TraitItemFn>>,
}
fn split_items(items: Vec<TraitItem>, trait_ident: &Ident) -> SplitItems {
    let mut split_items = SplitItems::default();
    for (order_number, item) in items.into_iter().enumerate() {
        match item {
            TraitItem::Const(trait_item_const) => split_items.assoc_constants.push(Ordered::new(
                order_number,
                TraitItemConstSyntax {
                    corresponding_generic_param_path: path::from_ident(format_ident!(
                        "{}_{}",
                        trait_ident,
                        trait_item_const.ident
                    )),
                    item: trait_item_const,
                },
            )),
            TraitItem::Fn(trait_item_fn) => {
                if signature::is_associated(&trait_item_fn.sig) {
                    split_items
                        .associated_fns
                        .push(Ordered::new(order_number, trait_item_fn))
                } else {
                    split_items
                        .static_fns
                        .push(Ordered::new(order_number, trait_item_fn))
                }
            }
            TraitItem::Type(trait_item_type) => split_items.assoc_types.push(Ordered::new(
                order_number,
                TraitItemTypeSyntax {
                    corresponding_generic_param_path: path::from_ident(format_ident!(
                        "{}_{}",
                        trait_ident,
                        trait_item_type.ident
                    )),
                    item: trait_item_type,
                },
            )),
            TraitItem::Macro(_) => panic!("Macro invocations inside trait are not supported"),
            TraitItem::Verbatim(_) => panic!("Verbatim trait items are not supported"),
            _ => panic!("Unexpected trait item: {}", item.to_token_stream()),
        }
    }

    return split_items;
}

fn map_trait_item_fn_to_fn_syntax(
    trait_item_fn: TraitItemFn,
    trait_syntax_as_fn_owner: &TraitSyntaxAsFnOwner,
) -> FnSyntax {
    let result = fn_syntax::prepare(fn_syntax::Params {
        attributes: trait_item_fn.attrs,
        visibility: Visibility::Inherited,
        signature: trait_item_fn.sig,
        maybe_base_impl: trait_item_fn.default.map(Box::new),
        maybe_owner: Some(trait_syntax_as_fn_owner),
        maybe_target_path: None,
    });
    return result;
}

fn merge_generics_with_assoc_generics(
    trait_ident: &Ident,
    mut generics: Generics,
    assoc_types: &[Ordered<TraitItemTypeSyntax>],
    assoc_constants: &[Ordered<TraitItemConstSyntax>],
) -> Generics {
    let assoc_types_as_generic_parameters = assoc_types.iter().map(|ordered| {
        ordered.ref_map(|x| {
            let result = TypeParam {
                attrs: Vec::new(),
                ident: format_ident!("{}_{}", trait_ident, x.item.ident),
                colon_token: None,
                bounds: x.item.bounds.clone(),
                default: x.item.default.clone(),
            };
            return GenericParam::Type(result);
        })
    });
    let assoc_constants_as_generic_parameters = assoc_constants.iter().map(|ordered| {
        ordered.ref_map(|x| {
            let const_ident = format_ident!("{}_{}", trait_ident, x.item.ident);
            let span = const_ident.span();
            let result = ConstParam {
                attrs: Vec::new(),
                const_token: Token![const](span),
                ident: const_ident,
                colon_token: Token![:](span),
                ty: x.item.ty.clone(),
                default: x.item.default.clone(),
            };
            return GenericParam::Const(result);
        })
    });
    let mut generic_parameters: Vec<_> = assoc_types_as_generic_parameters
        .chain(assoc_constants_as_generic_parameters)
        .collect();
    generic_parameters.sort_by_key(|a| a.order_number);
    generics
        .params
        .extend(generic_parameters.into_iter().map(|x| x.value));
    return generics;
}

struct TraitSyntaxAsFnOwner<'a> {
    pub trait_ident: &'a Ident,
    pub generics: &'a Generics,
}
impl<'a> IFnOwner for TraitSyntaxAsFnOwner<'a> {
    fn maybe_ident(&self) -> Option<&Ident> {
        None
    }

    fn format_name(&self) -> String {
        self.trait_ident.to_string()
    }

    fn generics(&self) -> &Generics {
        self.generics
    }
}
