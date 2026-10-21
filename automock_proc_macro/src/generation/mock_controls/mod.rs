pub mod models {
    mod setup;
    mod static_setup;

    pub(crate) use setup::*;
    pub(crate) use static_setup::*;
}

mod common {
    pub(crate) mod args_checker_stmt;
    pub(crate) mod fn_configurator_path;
    pub(crate) mod setup_struct;
}

use common::*;

pub mod fn_static_setup;
pub mod setup;
pub mod setup_impl;
pub mod static_setup;
pub mod fn_static_verify_calls;