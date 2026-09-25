pub mod models {
    mod argument;
    mod fn_syntax;
    mod i_fn_owner;

    pub(crate) use argument::*;
    pub(crate) use fn_syntax::*;
    pub(crate) use i_fn_owner::*;
}

mod common;

pub mod fn_syntax;
