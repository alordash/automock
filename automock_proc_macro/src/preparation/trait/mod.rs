pub mod models {
    mod trait_item_const_syntax;
    mod trait_item_type_syntax;
    mod trait_syntax;

    pub(crate) use trait_item_const_syntax::*;
    pub(crate) use trait_item_type_syntax::*;
    pub(crate) use trait_syntax::*;
}

pub mod trait_syntax;
