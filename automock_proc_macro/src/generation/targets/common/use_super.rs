use proc_macro2::{Ident, Span};
use syn::*;

pub(crate) fn new(span: Span) -> ItemUse {
    let result = ItemUse {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        use_token: Token![use](span),
        leading_colon: None,
        tree: UseTree::Path(UsePath {
            ident: Ident::new("super", span),
            colon2_token: Token![::](span),
            tree: Box::new(UseTree::Glob(UseGlob {
                star_token: Token![*](span),
            })),
        }),
        semi_token: Token![;](span),
    };
    return result;
}
