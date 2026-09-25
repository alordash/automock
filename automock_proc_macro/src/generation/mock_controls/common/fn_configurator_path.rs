use crate::common::*;
use crate::generation::common::*;
use crate::generation::fn_info::models::*;
use not_enough_syntax::*;
use proc_macro2::Span;
use syn::spanned::Spanned;
use syn::*;

pub(crate) fn new(
    span: Span,
    mock_struct_path: Path,
    fn_info: &FnInfo,
    generic_arguments: &generic_arguments::Result,
    lifetime: Lifetime,
    maybe_owner_type: Option<Type>,
) -> Path {
    let mock_struct_type = Type::Path(TypePath {
        attrs: Vec::new(),
        qself: None,
        path: mock_struct_path.clone(),
    });
    let mock_arg_type = match &fn_info.maybe_self_type {
        Some(receiver) => match &receiver.kind {
            ReceiverKind::Reference(and_token, _, mutability) => Type::Reference(TypeReference {
                attrs: Vec::new(),
                and_token: *and_token,
                lifetime: None,
                mutability: *mutability,
                elem: Box::new(mock_struct_type),
            }),
            ReceiverKind::Typed(_, target_type) => {
                normalization::normalize_in_type(*target_type.clone(), &mock_struct_path)
            }
            _ => mock_struct_type,
        },
        _ => mock_struct_type,
    };
    let result = path::new_generics_global(
        span,
        for_generated::new("FnConfigurator"),
        [
            GenericArgument::Lifetime(lifetime),
            generic_arguments.mock_generic_argument.clone(),
            GenericArgument::Type(maybe_owner_type.unwrap_or_else(|| Type::Path(self_type(span)))),
            GenericArgument::Type(Type::Tuple(fn_info.arg_refs_tuple.clone())),
            GenericArgument::Type(match &fn_info.return_type {
                ReturnType::Default => void_type(span),
                ReturnType::Type(_, return_type) => {
                    r#type::replace_anonymous_lifetimes_in_references(
                        *return_type.clone(),
                        &patch_lifetime::new(return_type.span()),
                    )
                }
            }),
            GenericArgument::Type(mock_arg_type),
            generic_arguments.has_return_value_argument.clone(),
            generic_arguments.supports_base_calling_argument.clone(),
            generic_arguments.passes_mock_to_callback_argument.clone(),
        ],
    );
    return result;
}
