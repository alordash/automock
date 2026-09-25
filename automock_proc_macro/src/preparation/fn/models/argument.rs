use syn::*;

pub(crate) struct Argument {
    pub source_pat_type: PatType,
    pub ident_pat_type: PatType,
    pub ident: Ident,
    pub ptr_style_type: Box<Type>,
    pub ref_style_type: Box<Type>,
    pub generic_arg_style_type: Box<Type>,
    pub control_fn_arg: FnArg,
    pub is_impl_trait: bool,
}

pub(crate) trait IArgumentTypesCloner {
    fn iter_generics_style_types(&self) -> impl Iterator<Item = Type>;
}

impl IArgumentTypesCloner for Vec<Argument> {
    fn iter_generics_style_types(&self) -> impl Iterator<Item = Type> {
        let result = self.iter().map(|x| *x.generic_arg_style_type.clone());
        return result;
    }
}
