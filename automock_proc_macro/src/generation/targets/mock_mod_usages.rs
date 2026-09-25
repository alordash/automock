use crate::generation::targets::models::*;
use proc_macro2::Span;
use syn::*;

pub(crate) fn new(span: Span) -> MockModUsages {
    let use_super = ItemUse {
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

    let result = MockModUsages { use_super };
    return result;
}
