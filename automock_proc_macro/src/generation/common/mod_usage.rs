use not_enough_syntax::attributes;
use syn::*;

pub(crate) fn new<const N: usize>(
    visibility: Visibility,
    mod_ident: Ident,
    target_idents: [Ident; N],
) -> ItemUse {
    let span = mod_ident.span();
    let result = ItemUse {
        attrs: Vec::new(),
        vis: visibility,
        use_token: Token![use](span),
        leading_colon: None,
        tree: UseTree::Path(UsePath {
            ident: mod_ident.clone(),
            colon2_token: Token![::](mod_ident.span()),
            tree: Box::new(if N == 1 {
                UseTree::Name(UseName {
                    ident: target_idents
                        .into_iter()
                        .next()
                        .expect("`target_idents` length must be 1"),
                })
            } else {
                UseTree::Group(UseGroup {
                    brace_token: token::Brace(span),
                    items: target_idents
                        .into_iter()
                        .map(|ident| UseTree::Name(UseName { ident }))
                        .collect(),
                })
            }),
        }),
        semi_token: Token![;](span),
    };
    return result;
}

pub(crate) fn new_all(mod_ident: Ident) -> ItemUse {
    new_core(mod_ident, false)
}

#[inline]
fn new_core(mod_ident: Ident, public: bool) -> ItemUse {
    let span = mod_ident.span();
    let (attrs, vis) = if public {
        (
            vec![attributes::allow_unreachable_pub(span)],
            Visibility::Public(Token![pub](span)),
        )
    } else {
        (Vec::new(), Visibility::Inherited)
    };
    let result = ItemUse {
        attrs,
        vis,
        use_token: Token![use](span),
        leading_colon: None,
        tree: UseTree::Path(UsePath {
            ident: mod_ident.clone(),
            colon2_token: Token![::](mod_ident.span()),
            tree: Box::new(UseTree::Glob(UseGlob {
                star_token: Token![*](span),
            })),
        }),
        semi_token: Token![;](span),
    };
    return result;
}
