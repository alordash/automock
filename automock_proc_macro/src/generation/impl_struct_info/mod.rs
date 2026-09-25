pub mod models {
    mod impl_struct_info;

    pub(crate) use impl_struct_info::*;
}

mod generation;

pub(crate) use generation::*;
