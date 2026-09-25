use proc_macro2::{Ident, Span};
use syn::punctuated::Punctuated;
use syn::*;

pub(crate) fn new(path_part: &str) -> [&str; 3] {
    ["automock", "for_generated", path_part]
}

pub(crate) fn new2<'a>(path_part_1: &'a str, path_part_2: &'a str) -> [&'a str; 4] {
    ["automock", "for_generated", path_part_1, path_part_2]
}

pub(crate) fn punctuated<P: Default, const N: usize>(
    span: Span,
    items: [PathSegment; N],
) -> Punctuated<PathSegment, P> {
    [
        PathSegment {
            ident: Ident::new("automock", span),
            arguments: PathArguments::None,
        },
        PathSegment {
            ident: Ident::new("for_generated", span),
            arguments: PathArguments::None,
        },
    ]
    .into_iter()
    .chain(items)
    .collect()
}

pub(crate) fn glob_usage(span: Span, target: &str) -> ItemUse {
    let result = ItemUse {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        use_token: Token![use](span),
        leading_colon: Some(Token![::](span)),
        tree: UseTree::Path(UsePath {
            ident: Ident::new("automock", span),
            colon2_token: Token![::](span),
            tree: Box::new(UseTree::Path(UsePath {
                ident: Ident::new("for_generated", span),
                colon2_token: Token![::](span),
                tree: Box::new(UseTree::Path(UsePath {
                    ident: Ident::new(target, span),
                    colon2_token: Token![::](span),
                    tree: Box::new(UseTree::Glob(UseGlob {
                        star_token: Token![*](span),
                    })),
                })),
            })),
        }),
        semi_token: Token![;](span),
    };
    return result;
}
