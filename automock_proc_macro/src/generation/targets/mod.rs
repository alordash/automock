pub(crate) mod models {
    mod mock_mod;
    mod mock_mod_usages;

    pub(crate) use mock_mod::*;
    pub(crate) use mock_mod_usages::*;
}

mod common;
pub mod r#fn;
pub mod impl_struct;
pub mod impl_trait_for_struct;
pub mod r#struct;
pub mod r#trait;

mod mock_mod_usages;
