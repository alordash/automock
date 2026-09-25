use crate::common::models::*;
use crate::generation::fn_info::models::*;
use crate::generation::mock_struct::models::*;
use crate::generation::mock_struct::*;
use not_enough_syntax::attributes;
use proc_macro2::Span;
use syn::*;

pub(crate) fn generate(
    ctx: &Context,
    source_span: Span,
    fn_info: &FnInfo,
    mock_struct_path: Path,
    mod_ident: Ident,
    maybe_base_fn_ident: Option<Ident>,
) -> ItemFn {
    let block = static_fn_block::generate(
        ctx,
        source_span,
        static_fn_block::Params {
            mock_struct_path,
            fn_info,
            base_fn_kind: match maybe_base_fn_ident {
                Some(x) => BaseFnKind::StaticFn(x),
                None => BaseFnKind::None,
            },
            mod_ident,
            for_struct: false,
            maybe_base_trait_ident: None,
            use_mod_path_for_call_stmt: false,
        },
    );

    let mut attributes = fn_info.attributes.clone();
    attributes.push(attributes::allow_clippy(source_span, "useless_transmute"));
    let result = ItemFn {
        attrs: attributes,
        vis: fn_info.visibility.clone(),
        modifiers: FnModifiers::default(),
        sig: fn_info.source_signature.clone(),
        block: Box::new(block),
    };
    return result;
}
