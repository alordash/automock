use not_enough_syntax::*;
use proc_macro2::Span;
use syn::*;

pub(crate) struct Params<'a> {
    pub generics: &'a Generics,
    pub maybe_argument_types: Option<Vec<Type>>,
}
pub(crate) fn new(
    span: Span,
    Params {
        generics,
        maybe_argument_types,
    }: Params,
) -> Type {
    let result = Type::Path(TypePath {
        attrs: Vec::new(),
        qself: None,
        path: path::new_generics_global(
            span,
            ["core", "marker", "PhantomData"],
            [GenericArgument::Type(Type::Tuple(TypeTuple {
                attrs: Vec::new(),
                paren_token: token::Paren(span),
                elems: generics
                    .params
                    .iter()
                    .filter_map(|x| match x {
                        GenericParam::Lifetime(lifetime) => Some(Type::Reference(TypeReference {
                            attrs: Vec::new(),
                            and_token: Token![&](span),
                            lifetime: Some(lifetime.lifetime.clone()),
                            mutability: None,
                            elem: Box::new(void_type(span)),
                        })),
                        GenericParam::Type(ty) => Some(Type::Path(r#type::box_of(
                            span,
                            Type::Path(TypePath {
                                attrs: Vec::new(),
                                qself: None,
                                path: path::from_ident(ty.ident.clone()),
                            }),
                        ))),
                        _ => None,
                    })
                    .chain(
                        maybe_argument_types
                            .into_iter()
                            .flat_map(|arguments| arguments.into_iter()),
                    )
                    .collect(),
            }))],
        ),
    });
    return result;
}
