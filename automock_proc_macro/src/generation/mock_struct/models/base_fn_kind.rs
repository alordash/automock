use proc_macro2::Ident;

pub(crate) enum BaseFnKind {
    None,
    StaticFn(Ident),
    Associated(Ident),
}
