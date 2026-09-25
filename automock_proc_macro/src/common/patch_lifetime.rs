use crate::constants;
use not_enough_syntax::punctuated;
use proc_macro2::Span;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::*;

pub(crate) fn new(span: Span) -> Lifetime {
    let result = Lifetime::new(constants::DEFAULT_ARG_LIFETIME_IDENT_STR, span);

    return result;
}

pub(crate) fn prepend_to_generics(mut generics: Generics) -> Generics {
    generics.params.insert(
        0,
        GenericParam::Lifetime(LifetimeParam {
            attrs: Vec::new(),
            lifetime: new(generics.span()),
            colon_token: None,
            bounds: Punctuated::new(),
        }),
    );
    return generics;
}

pub(crate) fn prepend_to_path(mut path: Path) -> Path {
    let span = path.span();
    if let Some(last_segment) = path.segments.last_mut() {
        let argument = GenericArgument::Lifetime(new(span));
        if let PathArguments::AngleBracketed(existing) = &mut last_segment.arguments {
            existing.args.insert(0, argument);
        } else {
            let new_arguments = AngleBracketedGenericArguments {
                colon2_token: Some(Token![::](span)),
                lt_token: Token![<](span),
                args: punctuated([argument]),
                gt_token: Token![>](span),
            };
            last_segment.arguments = PathArguments::AngleBracketed(new_arguments);
        };
    }
    return path;
}

pub(crate) fn revert_in_first_generic_arg(path: &mut Path) {
    if let Some(automock_anonymous_lifetime) = path
        .segments
        .last_mut()
        .and_then(|x| match &mut x.arguments {
            PathArguments::AngleBracketed(generics) => {
                generics
                    .args
                    .first_mut()
                    .map(|first_generic_arg| match first_generic_arg {
                        GenericArgument::Lifetime(lifetime_param)
                            if lifetime_param.ident == constants::DEFAULT_ARG_LIFETIME =>
                        {
                            Some(lifetime_param)
                        }
                        _ => None,
                    })
            }
            _ => None,
        })
        .flatten()
    {
        automock_anonymous_lifetime.ident = Ident::new("_", automock_anonymous_lifetime.span());
    }
}
