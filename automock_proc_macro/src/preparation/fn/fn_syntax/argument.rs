use crate::common::*;
use crate::preparation::r#fn::models::*;
use crate::preparation::r#fn::*;
use not_enough_syntax::*;
use proc_macro2::Span;
use syn::spanned::Spanned;
use syn::*;

pub(crate) fn new((number, source_pat_type): (usize, PatType)) -> Argument {
    let mut pat_ty = source_pat_type.clone();
    let impl_trait_replacement_result =
        normalization::replace_impl_trait_with_box_dyn_trait(*pat_ty.ty);
    *pat_ty.ty = impl_trait_replacement_result.ty;
    let is_impl_trait = impl_trait_replacement_result.is_impl_trait;
    let ident = prepare_ident(number, &pat_ty);
    let ident_pat_type = common::replace_arg_pat_with_ident(pat_ty.clone(), ident.clone());

    let ref_style_type = r#type::replace_anonymous_lifetimes_in_references(
        ident_pat_type.ty.clone(),
        &patch_lifetime::new(ident_pat_type.span()),
    );

    let ptr_style_type = r#type::replace_references_with_pointers(ref_style_type.clone());

    let generic_arg_style_type =
        r#type::replace_anonymous_references_with_pointers(ref_style_type.clone());

    let control_fn_arg = generate_control_fn_arg(
        ident.span(),
        ident_pat_type.pat.clone(),
        ref_style_type.clone(),
    );

    let result = Argument {
        source_pat_type,
        ident_pat_type,
        ident,
        ptr_style_type,
        ref_style_type,
        generic_arg_style_type,
        control_fn_arg,
        is_impl_trait,
    };
    return result;
}

fn prepare_ident(number: usize, pat_type: &PatType) -> Ident {
    let result = match pat_type.pat.as_ref() {
        Pat::Ident(pat_ident) => pat_ident.ident.clone(),
        not_ident => Ident::new(&format!("__arg{number}"), not_ident.span()),
    };

    return result;
}

// Can not pass ptr_style_type because this is the type that gets exposed to user in `mock.received()`
fn generate_control_fn_arg(span: Span, pat: Box<Pat>, ref_style_type: Box<Type>) -> FnArg {
    let result = PatType {
        attrs: Vec::new(),
        pat,
        colon_token: Token![:](span),
        ty: Box::new(Type::ImplTrait(TypeImplTrait {
            attrs: Vec::new(),
            impl_token: Token![impl](span),
            bounds: punctuated([TypeParamBound::Trait(TraitBound {
                paren_token: None,
                modifiers: TraitBoundModifiers::default(),
                maybe: None,
                lifetimes: None,
                path: path::new_generics_global(
                    span,
                    for_generated::new("IntoArg"),
                    [GenericArgument::Type(*ref_style_type)],
                ),
            })]),
        })),
    };

    return FnArg::Typed(result);
}
