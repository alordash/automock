pub mod models {
    mod impl_trait_for_struct_info;

    pub(crate) use impl_trait_for_struct_info::*;
}

mod generation;

pub(crate) use generation::*;
