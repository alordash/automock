use crate::common::models::*;
use crate::preparation::r#fn::models::*;
use not_enough_syntax::*;
use quote::format_ident;
use syn::spanned::Spanned;
use syn::visit_mut::VisitMut;
use syn::*;

pub(crate) fn normalize_associated_items(
    associated_items_info: &AssociatedItemsInfo,
    mut signature: Signature,
    mut block: Block,
) -> (Signature, Block) {
    let mut associated_items_normalizer = AssociatedItemsNormalizer {
        associated_items_info,
    };
    associated_items_normalizer.visit_signature_mut(&mut signature);
    associated_items_normalizer.visit_block_mut(&mut block);
    return (signature, block);
}

pub(crate) fn normalize_associated_items_in_fn_syntax(
    trait_ident: &Ident,
    associated_items_info: &AssociatedItemsInfo,
    mut fn_syntax: FnSyntax,
) -> FnSyntax {
    let mut associated_items_renamer = AssociatedItemsRenamer {
        trait_ident,
        associated_items_info,
    };
    associated_items_renamer.visit_generics_mut(&mut fn_syntax.merged_generics);
    associated_items_renamer.visit_field_mut(&mut fn_syntax.generics_field);
    for argument in fn_syntax.arguments.iter_mut() {
        associated_items_renamer.visit_type_mut(&mut argument.ident_pat_type.ty);
        associated_items_renamer.visit_type_mut(&mut argument.ptr_style_type);
        associated_items_renamer.visit_type_mut(&mut argument.ref_style_type);
        associated_items_renamer.visit_type_mut(&mut argument.generic_arg_style_type);
        associated_items_renamer.visit_fn_arg_mut(&mut argument.control_fn_arg);
    }
    associated_items_renamer.visit_type_tuple_mut(&mut fn_syntax.arg_refs_tuple);
    if let Some(base_impl) = &mut fn_syntax.maybe_base_impl {
        associated_items_renamer.visit_block_mut(base_impl);
    }
    associated_items_renamer.visit_return_type_mut(&mut fn_syntax.return_type);
    return fn_syntax;
}

struct AssociatedItemsNormalizer<'a> {
    associated_items_info: &'a AssociatedItemsInfo,
}

impl<'a> VisitMut for AssociatedItemsNormalizer<'a> {
    fn visit_type_path_mut(&mut self, i: &mut TypePath) {
        if i.path.segments.get(1).is_some_and(|second_segment| {
            self.associated_items_info
                .associated_items_ident_strings
                .contains(&second_segment.ident.to_string())
        }) && let Some(first_segment) = i.path.segments.get_mut(0)
            && first_segment.ident == "Self"
        {
            let span = first_segment.span();
            *first_segment = self.associated_items_info.trait_path_segment.clone();
            i.qself = Some(QSelf {
                lt_token: Token![<](span),
                ty: Box::new(Type::Path(self_type(span))),
                position: 1,
                as_token: Some(Token![as](span)),
                gt_token: Token![>](span),
            })
        }
        visit_mut::visit_type_path_mut(self, i);
    }
}

struct AssociatedItemsRenamer<'a> {
    trait_ident: &'a Ident,
    associated_items_info: &'a AssociatedItemsInfo,
}

impl<'a> VisitMut for AssociatedItemsRenamer<'a> {
    fn visit_type_path_mut(&mut self, i: &mut TypePath) {
        if i.path.segments.get(1).is_some_and(|second_segment| {
            self.associated_items_info
                .associated_items_ident_strings
                .contains(&second_segment.ident.to_string())
        }) && let Some(first_segment) = i.path.segments.get_mut(0)
            && first_segment.ident == "Self"
        {
            let second_segment = &mut i.path.segments[1];
            second_segment.ident = format_ident!("{}_{}", self.trait_ident, second_segment.ident);
            second_segment.arguments = PathArguments::None;
            i.path.segments = i.path.segments.iter().skip(1).cloned().collect();
        }
        visit_mut::visit_type_path_mut(self, i);
    }
}
