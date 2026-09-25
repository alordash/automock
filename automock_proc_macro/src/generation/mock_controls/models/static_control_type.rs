use syn::*;

pub(crate) enum StaticControlType {
    Setup {
        mock_generic_argument: Box<GenericArgument>,
    },
    Received,
}
