use crate::common::models::*;
use crate::generation::fn_info::models::*;
use not_enough_syntax::*;
use proc_macro2::Span;
use syn::*;

pub(crate) struct Params<'a> {
    pub mock_struct_path: Path,
    pub fn_info: &'a FnInfo,
    pub remove_lifetime_generic_arguments: bool,
}
pub(crate) struct Result {
    pub mock_generic_argument: GenericArgument,
    pub has_return_value_argument: GenericArgument,
    pub supports_base_calling_argument: GenericArgument,
    pub passes_mock_to_callback_argument: GenericArgument,
}
pub(crate) fn new(
    ctx: &Context,
    span: Span,
    Params {
        mut mock_struct_path,
        fn_info,
        remove_lifetime_generic_arguments,
    }: Params,
) -> Result {
    if remove_lifetime_generic_arguments {
        mock_struct_path = path::remove_lifetime_generic_arguments(mock_struct_path);
    }
    let result = Result {
        mock_generic_argument: GenericArgument::Type(Type::Path(TypePath {
            attrs: Vec::new(),
            qself: None,
            path: mock_struct_path,
        })),
        has_return_value_argument: generic_argument::bool(
            span,
            match fn_info.return_type {
                ReturnType::Default => false,
                ReturnType::Type(_, _) => true,
            },
        ),
        supports_base_calling_argument: generic_argument::bool(
            span,
            ctx.support_base_calling && fn_info.maybe_base_impl.is_some(),
        ),
        passes_mock_to_callback_argument: generic_argument::bool(
            span,
            signature::is_associated(&fn_info.signature),
        ),
    };
    return result;
}
