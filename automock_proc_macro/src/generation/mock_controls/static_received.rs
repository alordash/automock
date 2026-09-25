use crate::common::models::*;
use crate::generation::common::*;
use crate::generation::fn_info::models::*;
use crate::generation::mock_controls::models::*;
use crate::generation::mock_controls::*;
use not_enough_syntax::*;
use proc_macro2::Span;
use std::borrow::Borrow;
use syn::*;

pub(crate) struct Params<'a, T: Borrow<FnInfo>> {
    pub ident: Ident,
    pub generics: Generics,
    pub generics_for_impl: Generics,
    pub maybe_argument_types: Option<Vec<Type>>,
    pub mock_struct_path: &'a Path,
    pub fn_infos: &'a [T],
    pub for_static_fn: bool,
    pub maybe_trait_ident: Option<Ident>,
}
pub(crate) fn generate<T: Borrow<FnInfo>>(
    ctx: &Context,
    source_span: Span,
    Params {
        ident,
        generics,
        generics_for_impl,
        maybe_argument_types,
        mock_struct_path,
        fn_infos,
        for_static_fn,
        maybe_trait_ident,
    }: Params<T>,
) -> StaticReceivedStruct {
    let item_struct = control_struct::new_static(
        source_span,
        ident,
        generics,
        maybe_argument_types,
        ControlType::Received,
        maybe_trait_ident,
    );
    let path = path::from_ident_with_generics(item_struct.ident.clone(), &item_struct.generics);
    let clone_impl = clone_impl::generate(
        source_span,
        generics_for_impl.clone(),
        path.clone(),
        &item_struct.fields,
    );
    let item_impl = received_impl::generate(
        ctx,
        source_span,
        received_impl::Params {
            received_struct_path: path.clone(),
            generics: generics_for_impl,
            mock_struct_path,
            fn_infos,
            for_static_fn,
            is_static: true,
            generate_fn_no_other_calls: true,
        },
    );

    let result = StaticReceivedStruct {
        path,
        item_struct,
        clone_impl,
        item_impl,
    };
    return result;
}
